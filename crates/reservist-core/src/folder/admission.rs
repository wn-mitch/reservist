use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::{AdmissionSourceContext, AuthoredFolderOption, PreparedSlate, cards::slate_cards};
use crate::{
    calendar::{CalendarBoard, DatedCapacityReservation},
    routing::AuthoredRoutingPolicy,
    time::Instant,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AdmissionContext {
    pub(crate) actor_id: String,
    pub(crate) authority_ids: Vec<String>,
    pub(crate) accessible_evidence_ids: Vec<String>,
    pub(crate) eligible_binding_ids: Vec<String>,
    pub(crate) routing_policy: AuthoredRoutingPolicy,
    pub(crate) now: Instant,
    pub(crate) calendar: CalendarBoard,
}

impl AdmissionContext {
    pub(crate) fn source_context(&self) -> AdmissionSourceContext {
        AdmissionSourceContext {
            actor_id: self.actor_id.clone(),
            authority_ids: self.authority_ids.clone(),
            accessible_evidence_ids: self.accessible_evidence_ids.clone(),
            routing_policy: self.routing_policy.clone(),
            eligible_binding_ids: self.eligible_binding_ids.clone(),
        }
    }
}

pub(crate) fn prepare_slate(
    folder_id: &str,
    options: &[AuthoredFolderOption],
    context: &AdmissionContext,
    reviewed: &AdmissionSourceContext,
) -> Result<PreparedSlate, String> {
    if options.is_empty() {
        return Err("Pencil at least one option before handoff.".into());
    }
    if folder_id.is_empty() || context.actor_id.is_empty() {
        return Err("folder admission requires folder and actor identities".into());
    }
    let current = context.source_context();
    if reviewed.actor_id != current.actor_id {
        return Err("This folder belongs to a different officeholder.".into());
    }
    let mut mutual_exclusions = BTreeSet::new();
    let mut binding_ids = BTreeSet::new();
    let mut effective_options = options.to_vec();
    for option in &mut effective_options {
        for reservation in &mut option.reservations {
            if let super::slate::BoundAction::EvidenceRoute {
                delivery_delay_minutes,
                ..
            } = &option.binding
            {
                reservation.starts_at = context.now.to_string();
                reservation.releases_at = context
                    .now
                    .add_minutes(*delivery_delay_minutes)
                    .map_err(|error| error.to_string())?
                    .to_string();
                continue;
            }
            let start =
                Instant::parse(&reservation.starts_at).map_err(|error| error.to_string())?;
            let release =
                Instant::parse(&reservation.releases_at).map_err(|error| error.to_string())?;
            if release <= context.now {
                return Err(format!(
                    "option {} has missed its delivery window",
                    option.option_id
                ));
            }
            reservation.starts_at = start.max(context.now).to_string();
        }
    }
    let options = effective_options.as_slice();
    let mut reservations = Vec::new();
    for option in options {
        validate_access(option, reviewed)?;
        validate_option(option, &current, &mut binding_ids, &mut mutual_exclusions)?;
        reservations.extend(
            option
                .reservations
                .iter()
                .enumerate()
                .map(|(index, reservation)| {
                    let starts_at = Instant::parse(&reservation.starts_at).map_err(|error| {
                        format!(
                            "option {} has invalid reservation start: {error}",
                            option.option_id
                        )
                    })?;
                    let releases_at =
                        Instant::parse(&reservation.releases_at).map_err(|error| {
                            format!(
                                "option {} has invalid reservation release: {error}",
                                option.option_id
                            )
                        })?;
                    Ok(DatedCapacityReservation {
                        reservation_id: format!(
                            "folder.{folder_id}.{}.{}",
                            option.option_id,
                            index + 1
                        ),
                        owner_id: reservation.owner_id.clone(),
                        allocation: reservation.allocation,
                        starts_at,
                        releases_at,
                        expected_payoff: reservation.expected_payoff.clone(),
                        release_condition: reservation.release_condition.clone(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?,
        );
    }

    // The cloned board is the small calendar/booking state. It uses exactly the
    // same batch validator as real admission and cannot mutate live capacity.
    let mut preview_calendar = context.calendar.clone();
    let released_reservation_ids = options
        .iter()
        .flat_map(|option| option.released_reservation_ids.iter().cloned())
        .collect::<Vec<_>>();
    for reservation_id in &released_reservation_ids {
        preview_calendar.release_reservation(reservation_id)?;
    }
    preview_calendar.reserve_batch(reservations.clone())?;
    Ok(PreparedSlate {
        folder_id: folder_id.into(),
        selected_option_ids: options
            .iter()
            .map(|option| option.option_id.clone())
            .collect(),
        actions: options
            .iter()
            .map(|option| option.binding.clone())
            .collect(),
        reservations,
        released_reservation_ids,
        source_context: reviewed.clone(),
        cards: slate_cards(options),
    })
}

fn validate_option(
    option: &AuthoredFolderOption,
    context: &AdmissionSourceContext,
    binding_ids: &mut BTreeSet<String>,
    mutual_exclusions: &mut BTreeSet<String>,
) -> Result<(), String> {
    validate_access(option, context)?;
    let binding_id = option.binding.binding_id();
    if !binding_ids.insert(binding_id.into()) {
        return Err(format!("folder slate repeats binding: {binding_id}"));
    }
    for group in &option.mutual_exclusion_groups {
        if group.is_empty() || !mutual_exclusions.insert(group.clone()) {
            return Err(format!(
                "folder slate has mutually exclusive option group: {group}"
            ));
        }
    }
    Ok(())
}

fn validate_access(
    option: &AuthoredFolderOption,
    context: &AdmissionSourceContext,
) -> Result<(), String> {
    let binding_id = option.binding.binding_id();
    if !context
        .eligible_binding_ids
        .iter()
        .any(|id| id == binding_id)
    {
        return Err(format!(
            "The reviewed binding is not eligible in this decision context: {binding_id}"
        ));
    }
    for evidence_id in &option.required_evidence_ids {
        if !context
            .accessible_evidence_ids
            .iter()
            .any(|id| id == evidence_id)
        {
            return Err(format!(
                "folder option {} lacks accessible evidence: {evidence_id}",
                option.option_id
            ));
        }
    }
    for routing_choice_id in &option.routing_choice_ids {
        if !context.routing_policy.scope_policies.iter().any(|scope| {
            scope
                .conflict_choices
                .iter()
                .any(|choice| choice.choice_id == *routing_choice_id)
        }) {
            return Err(format!(
                "folder option {} references unknown reviewed routing choice: {routing_choice_id}",
                option.option_id
            ));
        }
    }
    let authority_id = option.binding.authority_id();
    if !authority_id.is_empty() && !context.authority_ids.iter().any(|id| id == authority_id) {
        return Err(format!(
            "actor {} lacks required authority: {authority_id}",
            context.actor_id
        ));
    }
    Ok(())
}

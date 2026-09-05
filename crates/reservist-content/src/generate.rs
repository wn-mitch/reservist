use std::{fs, path::Path};

use reservist_core::phase::metadata;

use crate::ContentError;

/// Writes stable registry artifacts from the core declaration table.
pub fn write_handler_metadata(output_dir: &Path) -> Result<(), ContentError> {
    fs::create_dir_all(output_dir)?;
    write_csv(&output_dir.join("handler_registry.csv"))?;
    write_fidelity(&output_dir.join("fidelity_permissions.csv"))?;
    write_mermaid(&output_dir.join("phase_flow.mmd"))
}

fn write_fidelity(path: &Path) -> Result<(), ContentError> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer
        .write_record([
            "tier",
            "identity_clade",
            "required_state",
            "capabilities",
            "runtime_promotion_permitted",
        ])
        .map_err(|error| ContentError::new("io", error.to_string()))?;
    for row in reservist_core::fidelity::permission_matrix() {
        writer
            .write_record([
                row.tier.as_str(),
                &row.identity_clade,
                &row.required_state,
                &row.capabilities.join(";"),
                if row.runtime_promotion_permitted {
                    "true"
                } else {
                    "false"
                },
            ])
            .map_err(|error| ContentError::new("io", error.to_string()))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| ContentError::new("io", error.to_string()))?;
    fs::write(path, bytes)?;
    Ok(())
}

fn write_csv(path: &Path) -> Result<(), ContentError> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(Vec::new());
    writer
        .write_record([
            "handler_id",
            "input_keys",
            "phase",
            "reads",
            "writes",
            "bounded_loop_owner",
            "bounded_loop_stopping_condition",
            "execution_mode",
            "provider_slot",
        ])
        .map_err(|error| ContentError::new("io", error.to_string()))?;
    for handler in metadata() {
        writer
            .write_record([
                handler.handler_id,
                handler.input_keys.join(";"),
                handler.phase.to_string(),
                handler.reads.join(";"),
                handler.writes.join(";"),
                handler
                    .bounded_loop
                    .as_ref()
                    .map_or("", |loop_meta| &loop_meta.owner)
                    .to_owned(),
                handler
                    .bounded_loop
                    .as_ref()
                    .map_or("", |loop_meta| &loop_meta.stopping_condition)
                    .to_owned(),
                handler.execution_mode.unwrap_or_default(),
                handler.provider_slot.unwrap_or_default(),
            ])
            .map_err(|error| ContentError::new("io", error.to_string()))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| ContentError::new("io", error.to_string()))?;
    fs::write(path, bytes)?;
    Ok(())
}

fn write_mermaid(path: &Path) -> Result<(), ContentError> {
    let mut text = String::from("flowchart LR\n");
    let handlers = metadata();
    for handler in &handlers {
        let node = format!("P{}", handler.phase);
        text.push_str(&format!(
            "    {node}[\"{}: {}\"]\n",
            handler.phase,
            handler.input_keys.join("\\n")
        ));
    }
    for pair in handlers.windows(2) {
        text.push_str(&format!("    P{} --> P{}\n", pair[0].phase, pair[1].phase));
    }
    fs::write(path, text)?;
    Ok(())
}

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[error("{0}")]
pub(crate) struct DeliveryError(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeliveredRecord {
    delivery: Value,
    item: Value,
    read: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PlayerRecordStore {
    pub recipient_id: String,
    pub access_profile: String,
    records: BTreeMap<String, DeliveredRecord>,
    delivery_order: Vec<String>,
}

impl PlayerRecordStore {
    pub(crate) fn new(recipient_id: String, access_profile: String) -> Self {
        Self {
            recipient_id,
            access_profile,
            records: BTreeMap::new(),
            delivery_order: Vec::new(),
        }
    }
    /// A lawful Chair transition rebinds future delivery without discarding
    /// institutional records already delivered to the office.
    pub(crate) fn rebind_recipient(&mut self, recipient_id: String) {
        self.recipient_id = recipient_id;
    }
    pub(crate) fn deliver(&mut self, delivery: &Value, item: &Value) -> Result<(), DeliveryError> {
        self.deliver_item(delivery, item, "observation_id", "observation_id")
    }
    pub(crate) fn deliver_artifact(
        &mut self,
        delivery: &Value,
        item: &Value,
    ) -> Result<(), DeliveryError> {
        self.deliver_item(delivery, item, "item_id", "record_id")
    }

    fn deliver_item(
        &mut self,
        delivery: &Value,
        item: &Value,
        reference_key: &str,
        item_key: &str,
    ) -> Result<(), DeliveryError> {
        if delivery.get("recipient_id").and_then(Value::as_str) != Some(&self.recipient_id) {
            return Err(DeliveryError(
                "delivery recipient does not match player".into(),
            ));
        }
        if delivery.get("access_scope").and_then(Value::as_str) != Some(&self.access_profile) {
            return Err(DeliveryError(
                "delivery access scope does not match player".into(),
            ));
        }
        let record_id = item
            .get(item_key)
            .and_then(Value::as_str)
            .ok_or_else(|| DeliveryError(format!("item requires {item_key}")))?;
        if delivery.get(reference_key).and_then(Value::as_str) != Some(record_id) {
            return Err(DeliveryError(
                "delivery and item references disagree".into(),
            ));
        }
        if self.records.contains_key(record_id) {
            return Err(DeliveryError(format!(
                "duplicate delivered item: {record_id}"
            )));
        }
        self.records.insert(
            record_id.into(),
            DeliveredRecord {
                delivery: delivery.clone(),
                item: item.clone(),
                read: false,
            },
        );
        self.delivery_order.push(record_id.into());
        Ok(())
    }

    pub(crate) fn delivered_records(&self) -> impl Iterator<Item = (&str, &Value, &Value)> {
        self.delivery_order.iter().map(|id| {
            let record = &self.records[id];
            (id.as_str(), &record.delivery, &record.item)
        })
    }
    pub(crate) fn list_delivered(&self) -> Vec<Value> {
        self.delivery_order
            .iter()
            .map(|id| json!(self.records[id]))
            .collect()
    }
    pub(crate) fn inspect(&mut self, id: &str) -> Result<(Value, bool), DeliveryError> {
        let record = self
            .records
            .get_mut(id)
            .ok_or_else(|| DeliveryError(format!("unknown delivered item: {id}")))?;
        let first_read = !record.read;
        record.read = true;
        Ok((json!(record), first_read))
    }
    pub(crate) fn contains(&self, id: &str) -> bool {
        self.records.contains_key(id)
    }
    pub(crate) fn unread_count(&self) -> usize {
        self.records.values().filter(|record| !record.read).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn foreign_delivery_is_rejected_and_only_first_read_transitions() {
        let mut store = PlayerRecordStore::new("player".into(), "scope".into());
        let item = json!({"observation_id":"observation.sample","observed_value":1});
        let mut delivery = json!({"recipient_id":"other","access_scope":"scope","observation_id":"observation.sample"});
        assert!(store.deliver(&delivery, &item).is_err());
        assert!(!store.contains("observation.sample"));
        delivery["recipient_id"] = "player".into();
        store.deliver(&delivery, &item).unwrap();
        assert_eq!(store.unread_count(), 1);
        let (record, first) = store.inspect("observation.sample").unwrap();
        assert!(first);
        assert_eq!(record["item"], item);
        assert_eq!(store.unread_count(), 0);
        assert!(!store.inspect("observation.sample").unwrap().1);
        assert!(store.inspect("state.hidden").is_err());
        assert!(store.deliver(&delivery, &item).is_err());
    }
    #[test]
    fn chair_rebinding_preserves_official_records_and_changes_future_recipient() {
        let mut store = PlayerRecordStore::new("person.outgoing".into(), "scope".into());
        let item = json!({"observation_id":"observation.official","observed_value":1});
        let outgoing_delivery = json!({
            "recipient_id":"person.outgoing",
            "access_scope":"scope",
            "observation_id":"observation.official"
        });
        store.deliver(&outgoing_delivery, &item).unwrap();
        store.rebind_recipient("person.successor".into());
        assert!(store.contains("observation.official"));
        let successor_item = json!({"observation_id":"observation.successor","observed_value":2});
        let successor_delivery = json!({
            "recipient_id":"person.successor",
            "access_scope":"scope",
            "observation_id":"observation.successor"
        });
        store.deliver(&successor_delivery, &successor_item).unwrap();
        assert!(store.contains("observation.successor"));
    }
}

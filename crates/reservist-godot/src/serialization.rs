use godot::prelude::*;
use serde_json::{Map, Number, Value};

pub fn value_to_dictionary(value: Value) -> Result<VarDictionary, String> {
    let Value::Object(object) = value else {
        return Err("serialized client response must be a JSON object".into());
    };
    let mut dictionary = VarDictionary::new();
    for (key, value) in object {
        dictionary.set(key.as_str(), &value_to_variant(value)?);
    }
    Ok(dictionary)
}

pub fn dictionary_to_value(dictionary: &VarDictionary) -> Result<Value, String> {
    let mut object = Map::new();
    for (key, value) in dictionary.iter_shared() {
        let key = key
            .try_to::<GString>()
            .map_err(|_| "Dictionary keys must be strings".to_owned())?;
        object.insert(key.to_string(), variant_to_value(value)?);
    }
    Ok(Value::Object(object))
}

fn value_to_variant(value: Value) -> Result<Variant, String> {
    Ok(match value {
        Value::Null => Variant::nil(),
        Value::Bool(value) => value.to_variant(),
        Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                value.to_variant()
            } else if let Some(value) = value.as_u64() {
                i64::try_from(value)
                    .map_err(|_| "JSON integer exceeds Godot's signed integer range".to_owned())?
                    .to_variant()
            } else {
                value
                    .as_f64()
                    .ok_or_else(|| "JSON number is not representable by Godot".to_owned())?
                    .to_variant()
            }
        }
        Value::String(value) => GString::from(&value).to_variant(),
        Value::Array(values) => {
            let mut array = VarArray::new();
            for value in values {
                array.push(&value_to_variant(value)?);
            }
            array.to_variant()
        }
        Value::Object(values) => value_to_dictionary(Value::Object(values))?.to_variant(),
    })
}

fn variant_to_value(value: Variant) -> Result<Value, String> {
    if value.is_nil() {
        return Ok(Value::Null);
    }
    if let Ok(value) = value.try_to::<bool>() {
        return Ok(Value::Bool(value));
    }
    if let Ok(value) = value.try_to::<i64>() {
        return Ok(Value::Number(value.into()));
    }
    if let Ok(value) = value.try_to::<f64>() {
        return Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| "Dictionary contains a non-finite number".into());
    }
    if let Ok(value) = value.try_to::<GString>() {
        return Ok(Value::String(value.to_string()));
    }
    if let Ok(value) = value.try_to::<VarArray>() {
        return value
            .iter_shared()
            .map(variant_to_value)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array);
    }
    if let Ok(value) = value.try_to::<VarDictionary>() {
        return dictionary_to_value(&value);
    }
    Err(format!(
        "Dictionary contains unsupported Godot value type {:?}",
        value.get_type()
    ))
}

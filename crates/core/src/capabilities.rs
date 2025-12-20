use serde_json::Value;

pub trait Capabilities {
    fn to_value(&self) -> Value;
}

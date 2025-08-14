use serde_json::json;

pub mod factorial;
pub mod fibonacci;

pub fn compute(n1: u128, n2: u128) -> String {
    let val = json!({"value": factorial::compute(n1) + fibonacci::compute(n2)});
    val.to_string()
}

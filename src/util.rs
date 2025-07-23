// https://claude.ai/chat/0ebcfe4b-4206-4999-b94d-af60407e40b2
use std::collections::HashMap;

pub fn hashmap_to_json_value(hashmap: HashMap<String, String>) -> serde_json::Value {
    serde_json::to_value(hashmap).unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
}

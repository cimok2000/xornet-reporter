use serde_json::Value;

pub fn bytes_to_kb(bytes: &Value) -> String {
    return (bytes.as_i64().unwrap() / 1024).to_string();
}

pub fn bytes_to_mb(bytes: &Value) -> String {
    return (bytes.as_i64().unwrap() / 1024 / 1024).to_string();
}

pub fn bytes_to_gb(bytes: &Value) -> String {
    return (bytes.as_i64().unwrap() / 1024 / 1024 / 1024).to_string();
}

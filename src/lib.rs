use serde_json::{Map, Value};

/// Insert a value that can be either flat or nested
/// - "name" → {"name": "Alice"} (flat)
/// - "user.name" → {"user": {"name": "Alice"}} (nested)
pub fn insert_value(root: &mut Map<String, Value>, path: &str, value: &str) {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.len() == 1 {
        // Simple field (no nesting)
        root.insert(path.to_string(), Value::String(value.to_string()));
        return;
    }

    // Navigate/create nested structure
    let mut current = root;

    for (i, part) in parts.iter().enumerate() {
        let is_last = i == parts.len() - 1;

        if is_last {
            // Insert the final value
            current.insert(part.to_string(), Value::String(value.to_string()));
        } else {
            // Create or get the nested object
            current = current
                .entry(part.to_string())
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
                .expect("Expected object");
        }
    }
}

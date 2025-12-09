use csv_to_json::*;  // This won't work yet - see step 4
use serde_json::json;
use serde_json::{Map, Value};

#[test]
fn test_flat_field() {
    let mut map = Map::new();
    insert_value(&mut map, "name", "Alice");
    insert_value(&mut map, "age", "25");
    
    let expected = json!({
        "name": "Alice",
        "age": "25"
    });
    assert_eq!(Value::Object(map), expected);
}

#[test]
fn test_nested_field() {
    let mut map = Map::new();
    insert_value(&mut map, "user.name", "Alice");
    
    let expected = json!({"user": {"name": "Alice"}});
    assert_eq!(Value::Object(map), expected);
}

#[test]
fn test_mixed_flat_and_nested() {
    let mut map = Map::new();
    insert_value(&mut map, "id", "1");
    insert_value(&mut map, "user.name", "Alice");
    insert_value(&mut map, "user.age", "25");
    insert_value(&mut map, "status", "active");
    
    let expected = json!({
        "id": "1",
        "user": {
            "name": "Alice",
            "age": "25"
        },
        "status": "active"
    });
    assert_eq!(Value::Object(map), expected);
}

#[test]
fn test_deeply_nested() {
    let mut map = Map::new();
    insert_value(&mut map, "user.contact.email", "test@example.com");
    insert_value(&mut map, "user.contact.phone", "555-0001");
    
    let expected = json!({
        "user": {
            "contact": {
                "email": "test@example.com",
                "phone": "555-0001"
            }
        }
    });
    assert_eq!(Value::Object(map), expected);
}
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::JsonSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum Message {
    Schema(Schema),
    Record(Record),
    State(State),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Record {
    stream: String,
    record: Value,
    time_extracted: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema {
    stream: String,
    schema: JsonSchema,
    key_properties: Vec<String>,
    bookmark_properties: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    value: Value,
}

#[cfg(test)]
pub mod tests {
    use serde_json::json;

    use crate::messages::{Message, Record, Schema, State};

    #[test]
    fn test_record() {
        let input =
            r#"{"type": "RECORD", "stream": "users", "record": {"id": 1, "name": "Chris"}}"#;

        let record: Message = serde_json::from_str(input).unwrap();

        let expected = Message::Record(Record {
            stream: "users".to_string(),
            record: json!({"id": 1, "name": "Chris"}),
            time_extracted: None,
        });
        assert_eq!(record, expected);
    }

    #[test]
    fn test_schema() {
        let input = r#"{"type": "SCHEMA", "stream": "users", "key_properties": ["id"], "schema": {"required": ["id"], "type": "object", "properties": {"id": {"type": "integer"}}}}"#;

        let schema: Message = serde_json::from_str(input).unwrap();

        let expected = Message::Schema(Schema {
            stream: "users".to_string(),
            key_properties: vec!["id".to_string()],
            schema: serde_json::from_value(
                json!({"required": ["id"], "type": "object", "properties": {"id": {"type": "integer"}}}),
            ).unwrap(),
            bookmark_properties: None,
        });
        assert_eq!(schema, expected);
    }

    #[test]
    fn test_state() {
        let input = r#"{"type": "STATE", "value": {"users": 2, "locations": 1}}"#;

        let schema: Message = serde_json::from_str(input).unwrap();

        let expected = Message::State(State {
            value: json!({"users": 2, "locations": 1}),
        });
        assert_eq!(schema, expected);
    }
}

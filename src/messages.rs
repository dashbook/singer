use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum Message {
    Schema(Schema),
    Record(Record),
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
    schema: Value,
    key_properties: Vec<String>,
    bookmark_properties: Option<Vec<String>>,
}

#[cfg(test)]
pub mod tests {
    use serde_json::json;

    use crate::messages::{Message, Record, Schema};

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
            schema: json!({"required": ["id"], "type": "object", "properties": {"id": {"type": "integer"}}}),
            bookmark_properties: None,
        });
        assert_eq!(schema, expected);
    }
}

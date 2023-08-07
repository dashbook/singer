use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum Message {
    Record(Record),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Record {
    stream: String,
    record: Value,
    time_extracted: Option<String>,
}

#[cfg(test)]
pub mod tests {
    use serde_json::json;

    use crate::messages::{Message, Record};

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
}

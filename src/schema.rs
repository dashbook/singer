use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub r#type: Type,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Type {
    Null,
    Boolean,
    Integer,
    Number(Number),
    String(StringType),
    Array(Array),
    Object(Object),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StringType {
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Number {
    pub multiple_of: Option<i32>,
    pub minimum: Option<i32>,
    pub exclusive_maximum: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    pub items: Box<Type>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Object {
    pub properties: HashMap<String, Type>,
    pub required: Option<Vec<String>>,
    pub additional_properties: Option<bool>,
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::schema::{Array, JsonSchema, Object, StringType, Type};

    #[test]
    fn test_null() {
        let input = r#"{"type": "null"}"#;

        let value: JsonSchema = serde_json::from_str(input).unwrap();

        let expected = JsonSchema {
            title: None,
            description: None,
            r#type: Type::Null,
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_boolean() {
        let input = r#"{"type": "boolean"}"#;

        let value: JsonSchema = serde_json::from_str(input).unwrap();

        let expected = JsonSchema {
            title: None,
            description: None,
            r#type: Type::Boolean,
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_integer() {
        let input = r#"{"type": "integer"}"#;

        let value: JsonSchema = serde_json::from_str(input).unwrap();

        let expected = JsonSchema {
            title: None,
            description: None,
            r#type: Type::Integer,
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_string() {
        let input = r#"{"type": "string"}"#;

        let value: JsonSchema = serde_json::from_str(input).unwrap();

        let expected = JsonSchema {
            title: None,
            description: None,
            r#type: Type::String(StringType::default()),
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_array() {
        let input = r#"{"type": "array", "items": {"type": "integer"}}"#;

        let value: JsonSchema = serde_json::from_str(input).unwrap();

        let expected = JsonSchema {
            title: None,
            description: None,
            r#type: Type::Array(Array {
                items: Box::new(Type::Integer),
            }),
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_object() {
        let input =
            r#"{"required": ["id"], "type": "object", "properties": {"id": {"type": "integer"}}}"#;

        let schema: JsonSchema = serde_json::from_str(input).unwrap();

        let expected = JsonSchema {
            title: None,
            description: None,
            r#type: Type::Object(Object {
                properties: HashMap::from_iter(vec![("id".to_string(), Type::Integer)]),
                required: Some(vec!["id".to_string()]),
                additional_properties: None,
            }),
        };
        assert_eq!(schema, expected);
    }
}

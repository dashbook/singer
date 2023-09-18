use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::JsonSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Catalog {
    pub streams: Vec<Stream>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metadata {
    pub metadata: Value,
    pub breadcrumb: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stream {
    pub stream: String,
    pub tap_stream_id: String,
    pub schema: JsonSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<Metadata>>,
}

#[cfg(test)]
pub mod tests {

    use std::collections::HashMap;

    use crate::{
        catalog::{Catalog, Stream},
        schema::{Compound, Object, Primitive, Type},
    };

    #[test]
    fn test_catalog() {
        let input = r#"{"streams": [{"stream": "tools","tap_stream_id": "tools","schema": {"type": "object","additionalProperties": false,"properties": {"id": {"type": "string"},"name": {"type": "string"},"updated_at": {"type": "string","format": "date-time"}}}}]}"#;

        let record: Catalog = serde_json::from_str(input).unwrap();

        let expected = Catalog {
            streams: vec![Stream {
                stream: "tools".to_string(),
                tap_stream_id: "tools".to_string(),
                schema: crate::schema::JsonSchema {
                    title: None,
                    description: None,
                    r#type: crate::schema::Type::Compound(Compound::Object(Object {
                        additional_properties: None,
                        required: None,
                        properties: HashMap::from_iter(vec![
                            (
                                "id".to_string(),
                                Type::Primitive {
                                    r#type: Primitive::String,
                                },
                            ),
                            (
                                "name".to_string(),
                                Type::Primitive {
                                    r#type: Primitive::String,
                                },
                            ),
                            (
                                "updated_at".to_string(),
                                Type::Primitive {
                                    r#type: Primitive::String,
                                },
                            ),
                        ]),
                    })),
                },
                table_name: None,
                metadata: None,
            }],
        };
        assert_eq!(record, expected);
    }
}

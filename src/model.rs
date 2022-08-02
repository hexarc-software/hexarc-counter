use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shield {
    schema_version: i32,
    label: String,
    message: String,
}

impl Shield {
    pub fn new(label: String, message: String) -> Self {
        Self {
            schema_version: 1,
            label,
            message,
        }
    }
}

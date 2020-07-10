/*
 * Open Service Broker API
 *
 * Extensions model
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "openapi_url")]
    pub openapi_url: String,
}

impl Extension {
    pub fn new(id: String, path: String, description: String, openapi_url: String) -> Extension {
        Extension {
            id: id,
            path: path,
            description: description,
            openapi_url: openapi_url,
        }
    }
}

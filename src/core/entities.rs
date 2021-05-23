use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddURLReq {
    pub url: String,
    #[serde(rename = "redirectionKey")]
    pub redirection_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveURLReq {
    #[serde(rename = "redirectionKey")]
    pub redirection_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableAddURL {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "fromURL")]
    pub from_url: String,
    #[serde(rename = "redirectionString")]
    pub redirection_string: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddURLResponse {
    pub to_url: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub db_key: String,
    pub db_type: String,
    pub base_url: String,
    pub secret_key: String,
}

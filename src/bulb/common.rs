use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Body<V> {
    pub commands: Vec<Command<V>>,
}

#[derive(Serialize)]
pub struct Command<V> {
    pub code: String,
    pub value: V,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T = serde_json::Value> {
    pub code: Option<i64>,
    pub msg: Option<String>,
    pub result: T,
    pub success: bool,
    pub t: i64,
    pub tid: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum CommandValue {
    Str(String),
    Int(i32),
    Map(serde_json::Value),
}

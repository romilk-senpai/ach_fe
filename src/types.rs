use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Board {
    pub name: String,
    pub category: String,
    pub slug: String,
    pub description: String,
    pub threads: Vec<Thread>,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Thread {
    pub id: String,
    pub num: i64,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub timestamp: i64,
    pub board: String,
}
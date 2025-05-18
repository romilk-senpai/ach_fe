use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Board {
    pub category_id: i32,
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct BoardExt {
    pub category_id: i32,
    pub id: i32,
    pub name: String,
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

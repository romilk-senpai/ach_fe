use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct BoardInfo {
    #[serde(rename = "categoryName")]
    pub category_id: String,
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Board {
    #[serde(rename = "categoryName")]
    pub category_id: String,
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub threads: Vec<Thread>,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Thread {
    #[serde(rename = "opPost")]
    pub op_post: Post,
    #[serde(rename = "lastReplies")]
    pub last_replies: Vec<Post>,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct Post {
    pub author: String,
    // #[serde(rename = "createdAt")]
    // pub created_at: i64,
    pub id: i32,
    pub subject: Option<String>,
    pub content: String,
}

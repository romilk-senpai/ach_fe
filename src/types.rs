use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Board {
    pub name: String,
    pub category: String,
    pub slug: String,
}
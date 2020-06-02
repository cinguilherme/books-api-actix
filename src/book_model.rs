use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub page_count: i32,
    pub chapters_count: i32
}


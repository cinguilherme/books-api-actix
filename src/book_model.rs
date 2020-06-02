use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="book")]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub page_count: i32,
    pub chapters_count: i32
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="author")]
pub struct Author {
    pub id: i32,
    pub name: String
}


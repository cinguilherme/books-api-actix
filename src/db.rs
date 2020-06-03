use crate::book_model::{Book, Author};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_books(client: &Client)
    -> Result<Vec<Book>, io::Error> {

    let statements =
        client.prepare("select * from book").await.unwrap();

    let books = client.query(&statements, &[])
        .await
        .expect("Error")
        .iter()
        .map(|row| Book::from_row_ref(row).unwrap())
        .collect::<Vec<Book>>();

    Ok(books)
}

pub async fn get_authors(client: &Client)
    -> Result<Vec<Author>, io::Error> {

    let statement = client.prepare("select * from author").await.unwrap();

    let authors = client.query(&statement, &[])
        .await
        .expect("unnable to fetch authors")
        .iter()
        .map(|rw| Author::from_row_ref(rw).unwrap())
        .collect::<Vec<Author>>();
    Ok(authors)
}
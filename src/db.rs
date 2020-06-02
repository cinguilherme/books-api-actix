use crate::book_model::Book;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_books(client: &Client)
    -> Result<Vec<Book>, io::Error> {

    let statements =
        client.prepare("select * from books").await.unwrap();

    let books = client.query(&statements, &[])
        .await
        .expect("Error")
        .iter()
        .map(|row| Book::from_row_ref(row).unwrap())
        .collect::<Vec<Book>>();

    Ok(books)
}
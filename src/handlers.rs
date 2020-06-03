use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use crate::book_model::{Status, Book};
use deadpool_postgres::{Pool, Client};
use crate::db;
use serde::{Serialize};

#[get("/{id}/{name}/index.html")]
pub async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/about")]
pub async fn about() -> impl Responder {
    HttpResponse::Ok().json(Status { status: "Ok".to_string() })
}

#[get("/authors")]
pub async fn authors(dp_pool: web::Data<Pool>) -> impl Responder {
    let client:Client = dp_pool.get().await.expect("unable to get connection");
    let result = db::get_authors(&client).await;
    match result {
        Ok(authors) => HttpResponse::Ok().json(authors),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[get("/books")]
pub async fn books(db_pool: web::Data<Pool>)
    -> impl Responder {

    let client: Client = db_pool.get()
            .await.expect("Error connectiong to DB");
    let result = db::get_books(&client).await;

    match result {
        Ok(books) => {
            HttpResponse::Ok().json(books)
        },
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().into()
        }
    }
}

#[get("/books/{id}")]
pub async fn book(_id: web::Path<u32>) -> impl Responder {

    let test_book = Book { id: 1, title: "Crime and Passion".to_string(),
        pages: 10, chapters: 10 };

    HttpResponse::Ok().json(test_book)
}
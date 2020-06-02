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
    let mut authors:Vec<String> = Vec::new();
    authors.push("Ivan Gomes".to_string());
    let test_book = Book { id: 1, title: "Crime and Passion".to_string(),
        page_count: 10, chapters_count: 10 };

    HttpResponse::Ok().json(test_book)
}
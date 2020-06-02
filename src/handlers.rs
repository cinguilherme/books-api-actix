use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use crate::book_model::{Status, Book};

#[get("/{id}/{name}/index.html")]
pub async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/about")]
pub async fn about() -> impl Responder {
    HttpResponse::Ok().json(Status { status: "Ok".to_string() })
}

#[get("/books")]
pub async fn books() -> impl Responder {
    format!("Hello from the books!")
}

#[get("/books/{id}")]
pub async fn book(_id: web::Path<u32>) -> impl Responder {
    let mut authors:Vec<String> = Vec::new();
    authors.push("Ivan Gomes".to_string());
    let test_book = Book { id: 1, title: "Crime and Passion".to_string(),
        page_count: 10, chapters_count: 10 };

    HttpResponse::Ok().json(test_book)
}
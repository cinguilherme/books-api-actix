use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use crate::book_model::{Book, Status};
use crate::my_config::{Config};
use dotenv::{dotenv};

mod my_config;
mod book_model;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().json(Status { status: "Ok".to_string() })
}

#[get("/books")]
async fn books() -> impl Responder {
    format!("Hello from the books!")
}

#[get("/books/{id}")]
async fn book(_id: web::Path<u32>) -> impl Responder {
    let mut authors:Vec<String> = Vec::new();
    authors.push("Ivan Gomes".to_string());
    let test_book = Book { title: "Crime and Passion".to_string(),
        authors: authors, page_count: 10, chapters_count: 10 };

    HttpResponse::Ok().json(test_book)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let config = my_config::Config::from_env().unwrap();

    let addressBind = format!("{}:{}", config.server.host,
                              config.server.port).to_string();

    println!("Server up on the bind -> {}", addressBind);

    HttpServer::new(|| App::new()
        .service(index)
        .service(about)
        .service(books)
        .service(book))
        .bind(addressBind)?
        .run()
        .await
}

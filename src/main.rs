use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/about")]
async fn about() -> impl Responder {
    format!("Hello from about!")
}

#[get("/books")]
async fn books() -> impl Responder {
    format!("Hello from the books!")
}

#[get("/books/{id}")]
async fn book(_id: web::Path<u32>) -> impl Responder {
    format!("Hello from the book single result {}!", _id)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(about)
        .service(books)
        .service(book))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

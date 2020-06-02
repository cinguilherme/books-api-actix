use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use tokio_pg_mapper;
use tokio_pg_mapper_derive;
use deadpool_postgres;
use crate::book_model::{Book, Status};
use crate::my_config::{Configuration};
use dotenv::{dotenv};
use tokio_postgres::NoTls;

mod my_config;
mod book_model;
mod handlers;
mod db;

use handlers::*;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let config = my_config::Configuration::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let address_bind = format!("{}:{}", config.server.host,
                              config.server.port).to_string();


    println!("Server up on the bind -> {}", address_bind);


    HttpServer::new(move ||
        App::new()
            .data(pool.clone())
        .service(index)
        .service(about)
        .service(books)
        .service(book))
        .bind(address_bind)?
        .run()
        .await
}

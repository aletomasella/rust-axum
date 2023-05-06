#![allow(unused)]
use std::net::SocketAddr;

use axum::{Router, response::Html};
use axum::routing::{get};



#[tokio::main]
async fn main () {

    let routes_hello = Router::new().route("/hello", get(|| async { Html("<h1>Hello, World!</h1>") })     
);


    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
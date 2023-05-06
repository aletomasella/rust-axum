#![allow(unused)]
use std::net::SocketAddr;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{Router, response::Html};
use axum::routing::{get};
use serde::Deserialize;



#[tokio::main]
async fn main () {

    let routes_hello = Router::new().route("/hello", get(handler_hello)     
);

#[derive(Debug,Deserialize)]
struct HelloParams {
    name: Option<String>,
}

  async fn handler_hello(Query(params) : Query<HelloParams>) -> impl IntoResponse {
    print!("{:?}", params);
    let name = params.name.as_deref().unwrap_or("World");
    
        Html(format!("Hello, {}!", name))
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
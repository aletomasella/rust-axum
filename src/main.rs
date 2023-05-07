#![allow(unused)]
use std::net::SocketAddr;

pub use self::error::{Error, Result};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::{response::Html, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;
mod error;
mod web;

#[tokio::main]
async fn main() {
    let all_routes = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handler_hello))
            .route("/hello2/:name", get(handler_hello2))
    }

    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        print!("{:?}", params);
        let name = params.name.as_deref().unwrap_or("World");

        Html(format!("Hello, {}!", name))
    }

    async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
        Html(format!("Hello, {}!", name))
    }

    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new(".")))
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();
}

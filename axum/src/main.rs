use std::{fs::File, io::Read};

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
    serve, Json, Router,
};
use axum_template::{engine::Engine, RenderHtml};
use serde::{Deserialize, Serialize};
use tera::Tera;
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct Ping {
    hello: String,
}

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

type AppEngine = Engine<Tera>;

#[derive(Serialize)]
pub struct PageContext {
    name: String,
}

async fn root() -> &'static str {
    "hi"
}

async fn mirror(Json(payload): Json<Ping>) -> Json<Ping> {
    Json(payload)
}

async fn params(Path(id): Path<u32>, Query(query): Query<QueryParams>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("x-powered-by", "benchmark".parse().unwrap());

    (headers, format!("{} {}", id, query.name))
}

async fn ely() -> impl IntoResponse {
    let mut file = File::open("public/ely.png").expect("public/ely.jpg doesn't existed");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Read buffer");

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "image/png".parse().unwrap());

    (headers, axum::body::Bytes::from(buffer))
}

async fn page(engine: AppEngine, Query(query): Query<QueryParams>) -> impl IntoResponse {
    let context = PageContext { name: query.name };

    RenderHtml("page.html", engine, context)
}

#[tokio::main]
async fn main() {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);

            std::process::exit(1);
        }
    };

    tera.autoescape_on(vec![".html"]);

    let app = Router::new()
        .route("/", get(root))
        .route("/id/:id", get(params))
        .route("/json", post(mirror))
        .route("/ely.png", get(ely))
        .route("/page.html", get(page))
        .with_state(Engine::from(tera));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}

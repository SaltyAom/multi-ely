#[macro_use]
extern crate lazy_static;

use actix_files::NamedFile;
use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    App, HttpResponse, HttpServer,
};

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct IncomingBody {
    name: String,
}

struct AppData {
    tera: Tera,
}

#[get("/")]
async fn index() -> &'static str {
    "hi"
}

#[get("/id/{id}")]
async fn params(path: Path<String>, query: Query<QueryParams>) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("x-powered-by", "benchmark"))
        .body(format!("{} {}", path, query.name))
}

#[post("/json")]
async fn mirror(body: Json<IncomingBody>) -> Json<IncomingBody> {
    body
}

#[get("/ely.png")]
async fn ely() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("public/ely.png")?)
}

#[get("/page.html")]
async fn page(query: Query<QueryParams>, data: Data<AppData>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("name", &query.name);

    HttpResponse::Ok()
        .append_header(("content-type", "text/html"))
        .body(
            data.tera
                .render("page.html", &context)
                .expect("Invalid page"),
        )
}

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        tera.autoescape_on(vec![".html"]);

        tera
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(AppData { tera: TERA.clone() }))
            .service(index)
            .service(params)
            .service(mirror)
            .service(ely)
            .service(page)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

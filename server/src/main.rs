mod cli;

use crate::cli::Options;
use actix_cors::Cors;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;
use pyo3::prelude::{PyAnyMethods, PyModule};
use pyo3::Python;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NlpText {
    text: String,
}

#[get("/nlp")]
async fn nlp(data: web::Query<NlpText>) -> impl Responder {
    web::Json(polytonize::polytonize_text(&data.text))
}

pub fn setup_python_paths() {
    let cwd = std::env::current_dir().unwrap();
    let lib_path = cwd.join(".venv/lib/python3.13/site-packages");

    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys").unwrap();
        let path = sys.getattr("path").unwrap();
        path.call_method1("append", (lib_path.display().to_string(),))
            .unwrap();
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let options = Options::parse();

    env_logger::Builder::new()
        .filter_level(options.verbosity.log_level_filter())
        .init();

    setup_python_paths();
    polytonize::initialize();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(["GET", "POST", "OPTIONS"])
            .allowed_header(CONTENT_TYPE);

        App::new().service(nlp).wrap(Logger::default()).wrap(cors)
    })
    .bind(("127.0.0.1", options.port))?
    .run()
    .await
}

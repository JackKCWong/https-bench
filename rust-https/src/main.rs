use std::{fs::File, io::BufReader};

use actix_files::Files;
use actix_web::{
    http::header::ContentType, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
    web::{Bytes,post}
};
use log::debug;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

/// simple handle
async fn index(bytes: Bytes) -> std::io::Result<HttpResponse> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => Ok(HttpResponse::Ok().body(text)),
        Err(_) => Ok(HttpResponse::BadRequest().into())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = load_rustls_config();

    log::info!("starting HTTPS server at https://localhost:8443");

    HttpServer::new(|| {
        App::new()
            // enable logger
            // .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .route("/", post().to(index))
    })
    .bind_rustls_021("127.0.0.1:8443", config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("../server.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("../server.key").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

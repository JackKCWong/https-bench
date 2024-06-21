use std::{
    fs::File,
    io::{self, BufReader},
};

use actix_web::{
    web::{post, Buf, Bytes},
    App, HttpResponse, HttpServer,
};
use rustls::{
    pki_types::PrivatePkcs8KeyDer,
    ServerConfig,
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use sha2::{Digest, Sha256};

/// simple handle
async fn index(bytes: Bytes) -> std::io::Result<HttpResponse> {
    let mut sha = Sha256::new();
    io::copy(&mut bytes.reader(), &mut sha)?;
    let r = sha.finalize();
    Ok(HttpResponse::Ok().body(hex::encode(r)))
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
    .bind_rustls_0_23("127.0.0.1:8443", config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("../server.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("../server.key").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys: Vec<PrivatePkcs8KeyDer> = pkcs8_private_keys(key_file).collect::<Result<Vec<_>, _>>().unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0).into()).unwrap()
}

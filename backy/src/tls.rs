use actix_files::Files;
use actix_web::{App, HttpServer};

pub async fn tls() -> std::io::Result<()> {
    let mut certs_file =
        std::io::BufReader::new(std::fs::File::open("cert.pem").unwrap());
    let mut key_file =
        std::io::BufReader::new(std::fs::File::open("key.pem").unwrap());

    let tls_certs = rustls_pemfile::certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
        .next()
        .unwrap()
        .unwrap();

    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            tls_certs,
            rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key),
        )
        .unwrap();

    HttpServer::new(|| App::new().service(Files::new("/w", "www")))
        .bind_rustls_0_23(("::", 444), tls_config)?
        // .bind("localhost:8989")?
        .run()
        .await
}

// [dependencies]
// actix-web = { version = "4", features = ["rustls-0_22"] }
// rustls = "0.22"
// rustls-pemfile = "2"
// actix-files = "0"

use actix_tls::rustls::TlsStream;
use actix_web::{dev::Extensions, rt::net::TcpStream, web, App, HttpServer};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::{
    any::Any,
    io::{self, BufReader},
};

async fn handle(message: web::ReqData<String>) -> String {
    println!("in handle");
    format!("message:{:#?}\n\n", message)
}

fn connect_func(connection: &dyn Any, data: &mut Extensions) {
    println!("in connect_func");
    if let Some(sock) = connection.downcast_ref::<TlsStream<TcpStream>>() {
        println!("any is TlsStream<TcpStream>");
        // let (sock, _) = sock.get_ref();
        data.insert("Hello World".to_string());
    } else {
        println!("any is other");
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("server.crt").unwrap());
    let key_file = &mut BufReader::new(File::open("ca.key").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(|| App::new().default_service(web::to(handle)))
        .on_connect(connect_func)
        .bind_rustls(("127.0.0.1", 8080), config)?
        .workers(1)
        .run()
        .await
}

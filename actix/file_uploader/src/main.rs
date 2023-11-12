use actix_web::{post, App, HttpResponse, HttpServer, Result};
use actix_multipart::Multipart;
use std::fs;
use std::io::Write;
use futures_util::StreamExt as _;

#[post("/upload")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
    while let Some(field) = payload.next().await {
        println!("Reached first spot");
        let mut field = field.unwrap();
        let file_name = field.name();
        println!("Reached intermediate spot");
        let mut file = fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
        .unwrap();
    
        println!("Reached 2nd spot");

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).unwrap();
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(upload_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
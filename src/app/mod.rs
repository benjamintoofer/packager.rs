use mime;
use actix_files::NamedFile;
use actix_cors::Cors;
use actix_web::{middleware, http, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_web::http::header::{ContentDisposition, DispositionType};
use std::path::{Path,PathBuf};

use crate::error::CustomError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// async fn manual_hello(req: HttpRequest) -> impl Responder {
//   let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//   println!("{:?}",Path::new("/output").join(&path));
//   match NamedFile::open(Path::new("/output").join(&path)) {
//       Ok(manifest) => {HttpResponse::Ok().content_type("application/x-mpegURL").body(manifest);}
//       Err(_) => {HttpResponse::NotFound();}
//   }
//   HttpResponse::Ok().body("Hey there!")
// }

async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("FILE: {:?}", Path::new("./output").join(&path));
    let file = NamedFile::open(Path::new("./output").join(path))?;
    let hls = "application/vnd.apple.mpegurl".parse::<mime::Mime>().unwrap();
     Ok(file
        .use_last_modified(true)
        .set_content_type(hls))
        // .set_content_disposition(ContentDisposition {
        //     disposition: DispositionType::Attachment,
        //     parameters: vec![],
        // }))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};
    println!("MAIN");
    HttpServer::new(|| {
      let cors = Cors::default()
              .allowed_origin("https://hls-js.netlify.app")
              // .allowed_methods(vec!["GET", "POST, OPTIONS"])
              // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE, http::header::ORIGIN])
              // .allowed_header(http::header::CONTENT_TYPE)
              // .allowed_header(http::header::RANGE)
              .allow_any_header()
              .allow_any_method()
              .max_age(3600);

      App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/{filename:.*}", web::get().to(index))
    })
        .bind(("0.0.0.0",8080))?
        .run()
        .await
}
use actix_web::{web, App, HttpServer, Result, Responder};
use serde::{Deserialize,Serialize};
use crate::assets::client::api::{ read_key};

mod assets;

#[derive(Deserialize)]
struct Info {
    ip: String,
}

#[derive(Deserialize,Serialize)]
struct Key {
    private_key:String,
    public_key:String
}

async fn index(info: web::Json<Info>) -> Result<impl Responder> {
    let (pb_key,pt_key) = read_key(info.ip.as_str()).await;
    let keys = Key{
        private_key:pt_key,
        public_key:pb_key
    };
    Ok(web::Json(keys))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Keygen run!");
    HttpServer::new(|| App::new()
        .route("/getIp", web::post().to(index)))
        .bind(("0.0.0.0", 2525))?
        .run()
        .await
}
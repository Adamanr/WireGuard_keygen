use actix_web::{post, get, web, App, HttpServer, Result, Responder};
use serde::{Deserialize,Serialize};
use crate::assets::client::api::{delete_client, read_key};

mod assets;

#[derive(Deserialize)]
struct Info {
    ip: String,
}

#[derive(Deserialize)]
struct PKey {
    key: String,
}

#[derive(Deserialize,Serialize)]
struct Key {
    private_key:String,
    public_key:String
}

#[post("/add_key")]
async fn add(info: web::Json<Info> ) -> Result<impl Responder> {
    if info.ip == "" {
        panic!("ip is empty!");
    }
    let (pb_key,pt_key) = read_key(info.ip.as_str()).await;
    let keys = Key{
        private_key:pt_key,
        public_key:pb_key
    };
    Ok(web::Json(keys))
}

#[get("/delete_key")]
async fn delete(info: web::Json<PKey> )-> Result<String>{
    delete_client(info.key.clone()).await;
    Ok(String::from("Удалено!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Keygen run!");
    HttpServer::new(|| {
        App::new().service(
            web::scope("")
                .service(add)
                .service(delete)
        )
    })
        .bind(("0.0.0.0", 2525))?
        .run()
        .await
}

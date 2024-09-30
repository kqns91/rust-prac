use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::db_connector;

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(get_technology_page))
        .bind((addr, port))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// マクロを使うことで HTTP メソッドとパスを指定した状態で関数を定義できる
#[get("/technologies/{tech_name}")]
async fn get_technology_page(tech_name: web::Path<String>) -> impl Responder {
    let conn = &mut db_connector::create_connection();
    let path = tech_name.to_string();
    let (tech, projs) =
        db_connector::get_technology_page_by_url_name(conn, &path).expect("NotFound");

    HttpResponse::Ok().content_type("text/html").body(tech.name)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("launching server...");
    webapi_mvp::apis::create_app("127.0.0.1", 8080).await
}

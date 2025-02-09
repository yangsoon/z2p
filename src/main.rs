use actix_web::{web, App, HttpServer, HttpRequest, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(greet))
    }).bind("127.0.0.1:8000")?.run().await
}
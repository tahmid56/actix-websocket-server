use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
mod ws;
mod api;
mod models;
mod helper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(api::send_message)
            .route("/ws", web::get().to(api::websocket_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// src/api.rs
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::ws::WebSocketSession;



#[derive(Deserialize)]
pub struct GenericRequest {
    pub body: serde_json::Value,
}

#[post("/send")]
pub async fn send_message(req: web::Json<GenericRequest>) -> impl Responder {
    println!("Body: {:?}", req.body);
    // Here, we're just sending back the "body" field of the request
    HttpResponse::Ok().json(req.body.clone())
}


pub async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> impl Responder{
    ws::start(WebSocketSession::new(), &req, stream)
}
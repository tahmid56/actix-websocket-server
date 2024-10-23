// src/ws.rs
use actix::{Actor, StreamHandler};
use actix_web::web;
use actix_web_actors::ws;

use std::time::{Duration, Instant};
use actix::prelude::*;

use crate::helper;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

pub struct WebSocketSession {
    pub last_heartbeat: Instant,
}

impl WebSocketSession {
    pub fn new() -> Self {
        WebSocketSession {
            last_heartbeat: Instant::now(),
        }
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);
    }
}

impl WebSocketSession {
    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > Duration::from_secs(10) {
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                match helper::extract_body_from_curl(&text) {
                    Some(body) => {
                        ctx.text(body)
                    },
                    None => {
                        ctx.text("Invalid cURL command")
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            _ => ctx.stop(),
        }
    }
}


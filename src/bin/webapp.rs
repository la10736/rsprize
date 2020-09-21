use std::sync::Mutex;

use anyhow::{Context, Result};

use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::StreamExt;
use members::read_accepted_rsvp;
use rand::prelude::{SliceRandom, StdRng};
use serde::Deserialize;

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct PrizesParams {
    n: Option<usize>,
}

async fn prizes(
    params: web::Query<PrizesParams>,
    mut body: web::Payload,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let mut members = read_accepted_rsvp(bytes.as_ref())?;
    members.sort_by_key(|m| m.name.clone());
    let mut rng = state.rng.lock().unwrap();
    members.shuffle(&mut *rng);

    Ok(HttpResponse::Ok().json(&members[0..params.n.unwrap_or(1)]))
}

struct State {
    rng: Mutex<StdRng>,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init();
    let port: u16 = std::env::var_os("RSPRIZE_PORT")
        .and_then(|p| p.into_string().ok())
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = format!("127.0.0.1:{}", port);

    let rng = rsprize::rng::build().context("Cannot build rng")?;

    HttpServer::new(move || {
        App::new()
            .data(State {
                rng: Mutex::new(rng.clone()),
            })
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/prizes", web::get().to(prizes))
    })
    .bind(&addr)?
    .run()
    .await
    .context("Server Error")
}

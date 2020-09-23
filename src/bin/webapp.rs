use std::sync::Mutex;

use actix_web::{
    get, middleware,
    web::{self, Query},
    App, HttpServer, Responder,
};
use env_logger;
use futures_util::StreamExt;
use rand::prelude::{SliceRandom, StdRng};
use serde::Deserialize;

#[get("/health_check")]
async fn index() -> impl Responder {
    ""
}

#[derive(Deserialize)]
struct PrizeArgs {
    n: Option<usize>,
}

#[get("/prizes")]
async fn prizes(
    args: Query<PrizeArgs>,
    mut body: web::Payload,
    rng: web::Data<Mutex<StdRng>>,
) -> impl Responder {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    let mut members = members::read_accepted_rsvp(bytes.as_ref()).unwrap();
    members.sort_by(|x, y| x.name.cmp(&y.name));
    let mut rng = rng.lock().unwrap();
    members.shuffle(&mut *rng);
    // members.shuffle(&mut rsprize::rng::build().unwrap());

    members.truncate(args.n.unwrap_or(1));
    web::Json(members)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port = std::env::var_os("RSPRIZE_PORT")
        .and_then(|p| p.into_string().ok())
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    HttpServer::new(move || {
        App::new()
            .data(Mutex::new(rsprize::rng::build().unwrap()))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(prizes)
    })
    .bind(&format!("127.0.0.1:{}", port))?
    .run()
    .await
}

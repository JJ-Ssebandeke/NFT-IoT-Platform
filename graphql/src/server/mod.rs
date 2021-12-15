mod routes;
use rocket::{build};
use routes::world;

pub async fn start() {
    build()
        .mount("/",routes![world])
        .launch().await;
}

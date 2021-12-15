#[macro_use] extern crate rocket;
mod server;
mod schema;

#[rocket::main]
async fn main() {
    server::start().await;
}

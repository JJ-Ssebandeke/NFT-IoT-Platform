#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;
mod server;
mod schema;

#[rocket::main]
async fn main() {
    server::start().await;
}

mod routes;
use rocket::{build, routes};
use routes::{graphql_playground,get_graphql_handler,post_graphql_handler};

use crate::schema;

pub async fn start() {

    let schema = schema::build_schema();
    build()
        .manage(schema)
        .mount("/", routes![graphql_playground, get_graphql_handler, post_graphql_handler])
        .launch().await;
}


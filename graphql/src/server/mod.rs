mod routes;
use rocket::{build, routes};
use routes::{graphql_playground,graphql_query,graphql_request};

use crate::schema;

pub async fn start() {

    let schema = schema::build_schema().finish();
    build()
        .manage(schema)
        .mount("/", routes![graphql_playground, graphql_query, graphql_request])
        .launch().await;
}


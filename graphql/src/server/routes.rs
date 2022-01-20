// graphql routes

use rocket::{response::content, State};
use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use crate::schema::Schema;

#[get("/")]
pub fn graphql_playground() -> content::Html<String> {
    juniper_rocket::playground_source("/graphql", None)
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(request: GraphQLRequest, schema: &State<Schema>,) -> GraphQLResponse {
    request.execute_sync(&*schema, &())

}

#[post("/graphql", data="<request>")]
pub fn post_graphql_handler(request: GraphQLRequest, schema: &State<Schema>,) -> GraphQLResponse {
    request.execute_sync(&*schema, &())
}
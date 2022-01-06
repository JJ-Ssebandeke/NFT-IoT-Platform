// graphql routes
use crate::schema::{Query, Mutation, Subscription};
use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, State};

pub type DappSchema = Schema<Query, Mutation, Subscription>;

#[get("/")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<DappSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(schema: &State<DappSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

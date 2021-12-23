mod devices;
mod blocks;
mod client;
mod admin;

use async_graphql::*;
#[derive(MergedObject, Default)]
pub struct Query(devices::DeviceQuery);
#[derive(MergedObject, Default)]
pub struct Mutation(client::ClientMutation, admin::AdminMutation);
#[derive(MergedSubscription, Default)]
pub struct Subscription(blocks::BlockSubsription);

//build schema
pub fn build_schema() -> SchemaBuilder<Query,Mutation,Subscription> {
      
    Schema::build(Query::default(), Mutation::default(), Subscription::default())
}

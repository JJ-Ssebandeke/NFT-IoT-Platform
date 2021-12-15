use juniper::{graphql_object, Context, EmptyMutation, EmptySubscription, RootNode,};

pub struct Query;

//#[graphql_object]
impl Query {
    
}

pub fn bulid_schema() -> RootNode<> {
    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );
    schema
}
use async_graphql::Subscription;
use web3::types::Block;

#[derive(Default)]
pub struct BlockSubsription;

#[Subscription]
impl BlockSubsription {
    async fn blocks(&self) -> impl Stream<Item = i32> {
        futures::stream::iter(10..20)
    }
}
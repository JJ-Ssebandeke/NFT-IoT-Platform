pub struct Subscriptions;

use futures::Stream;
use serde_json::{json, Value};
use tokio_stream::StreamExt;
use web3::{types::{BlockId, BlockNumber, Block, H256}, transports::Http, api::Eth, Web3, block_on};

#[graphql_subscription]
impl Subscriptions {

    async fn blocks(&self) -> impl Stream<Item = Value> {

        let http = Http::new("http://localhost:7545").unwrap();
        let web3 =  Web3::new(http);
        
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(20)))
        .map(move |_|{
            
            let block  = block_on(Eth::block(&web3.eth(),BlockId::Number(BlockNumber::Latest))).unwrap();
            let out = match block {
                 Some(x) => json!(x),
                 None => json!(Block::<H256>::default()),
            };
            json!(out)
        })
        
    }
    
}
use std::time::Duration;

use async_graphql::{Subscription};
use futures::Stream;
use serde_json::{json, Value};
use tokio_stream::StreamExt;
use web3::{types::{BlockId, BlockNumber, Block, H256}, transports::Http, api::Eth, Web3, block_on};


#[derive(Default)]
pub struct BlockSubsription;

#[Subscription]
impl BlockSubsription {
    async fn blocks(&self) -> impl Stream<Item = Value> {

        let http = Http::new("http://localhost:8545").unwrap();
        let web3 =  Web3::new(http);
        
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(10)))
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



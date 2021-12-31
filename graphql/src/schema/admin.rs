use async_graphql::{Object, Context};
use hex_literal::hex;
use web3::{contract::{Contract, Options}, transports::Http, error::Result};
use serde_json::{json, Value};

#[derive(Default)]
pub struct AdminMutation;

#[Object]
impl AdminMutation{
    async fn register(&self,ctx: &Context<'_>, device_id: String, _url: String , _address: String) -> Result<Value> {
        // register device
        let address = hex!("d028d24f16a8893bd078259d413372ac01580769").into();
        let smart_contract = ctx.data::<Contract<Http>>().unwrap();
        let tx = smart_contract.
        call_with_confirmations("registerDevice", (device_id,_url,_address),address,Options::default(),1).await?;
        Ok(json!(tx))
        
    }
}


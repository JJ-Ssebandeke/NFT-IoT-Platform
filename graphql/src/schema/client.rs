use async_graphql::{Object, Context};
use hex_literal::hex;
use serde_json::{Value, json};
use web3::{contract::{Contract, Options}, transports::Http, error::Result};
#[derive(Default)]
pub struct ClientMutations;

#[Object]
impl ClientMutations{
    async fn burrow_devices(&self, ctx: &Context<'_>, _to: String, _duration: i16, _devices_requested: i16) -> Result<Value> {
        // request devices
        let address = hex!("d028d24f16a8893bd078259d413372ac01580769").into();
        let smart_contract = ctx.data::<Contract<Http>>().unwrap();
        let tx = smart_contract.
        call_with_confirmations("mintTag", (_to,_duration,_devices_requested),address,Options::default(),1).await?;
        Ok(json!(tx))
    }

    async fn return_devices(&self,ctx: &Context<'_> ,_tag_holder: String, _device_returned: i16) -> Result<Value>{
        // return loaned devices
        let address = hex!("d028d24f16a8893bd078259d413372ac01580769").into();
        let smart_contract = ctx.data::<Contract<Http>>().unwrap();
        let tx = smart_contract.
        call_with_confirmations("burnTag", (_tag_holder,_device_returned),address,Options::default(),1).await?;
        Ok(json!(tx))
    }

}
use async_graphql::Object;
use hex_literal::hex;
use web3::{contract::Options, transports::Http, error::Result, types::H160};
use serde_json::{json, Value};

use super::helpers::_contract;

#[derive(Default)]
pub struct AdminMutation;

#[Object]
impl AdminMutation{
    async fn register(&self, device_id: String, _url: String , device_address: String) -> Result<Value> {
        
        let d: H160 = hex!("64e38Deee57C4573F3aD542859f3a84CD5DB56bD").into();
        //let hexd  = hex::decode(device_address)?;
        // register device
        let http = Http::new("http://localhost:7545")?;
        
        let address = hex!("64e38Deee57C4473F3aD542859f3a84CD5DB56bD");

        let contract = _contract(http.clone(), address.into(),include_bytes!("./abi/DeviceHub.json")).await.unwrap();

        let admin = hex!("1326ead98D613c9233D905a716d8105BE029e69D").into();
        let tx = contract.
        call_with_confirmations("registerDevice", (device_id,_url,d),admin,Options::default(), 1).await?;
        Ok(json!(tx))
        
    }
}


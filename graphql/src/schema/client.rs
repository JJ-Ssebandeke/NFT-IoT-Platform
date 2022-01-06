use async_graphql::Object;
use hex_literal::hex;
use serde_json::{Value, json};
use web3::{contract::Options, transports::Http, error::Result, types::U256};

use super::helpers::_contract;
#[derive(Default)]
pub struct ClientMutations;

#[Object]
impl ClientMutations{
    async fn burrow_devices(&self, _duration: String, _devices_requested: String) -> Result<Value> {
        // request devices
        let http = Http::new("http://localhost:7545")?;
        let address = hex!("202AF303B185d87C8b79659c5f69a89E6F5981Da");
        let _contract = _contract(http.clone(), address.into(), include_bytes!("./abi/DeviceTag.json")).await.unwrap();
        
        let ud = U256::from_dec_str(&_duration).unwrap();
        let udr = U256::from_dec_str(&_devices_requested).unwrap();
    
        let client_address = hex!("1F8CD3c544Bc1C225278dA0449CCD18006d93Ebe").into();  
        let tx = _contract.
        call_with_confirmations("mintTag", (ud,udr),client_address,Options::default(), 1).await.unwrap();
        Ok(json!(tx))
    }

    async fn return_devices(&self, _devices_returned: i16) -> Value{
        // return loaned devices
        let http = Http::new("http://localhost:7545").unwrap();
        let address = hex!("202AF303B185d87C8b79659c5f69a89E6F5981Da");
        let _contract = _contract(http.clone(), address.into(), include_bytes!("./abi/DeviceTag.json")).await.unwrap();
    
        let client_address = hex!("1F8CD3c544Bc1C225278dA0449CCD18006d93Ebe").into();
       
        let tx = _contract.
        call("burnTag", _devices_returned,client_address,Options::default()).await.unwrap();
        json!(tx)
    }

}
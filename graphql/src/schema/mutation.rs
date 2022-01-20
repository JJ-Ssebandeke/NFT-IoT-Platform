use hex_literal::hex;
use juniper::FieldResult;
use web3::{contract::Options, types::{H160, U256}};
use super::Contracts;

pub struct Mutations;

#[graphql_object(context = Contracts)]
impl Mutations{

    async fn register(ctx: &Contracts, device_id: String, _url: String , device_address: String) -> FieldResult<String> {
         // register device
        let d: H160 = hex!("64e38Deee57C4573F3aD542859f3a84CD5DB56bD").into();
       
        let admin = hex!("1326ead98D613c9233D905a716d8105BE029e69D").into();
        let tx = ctx.device_manager.
        call_with_confirmations("registerDevice", (device_id,_url,d),admin,Options::default(), 1).await?;
        let graphql_output = serde_json::to_string(&tx)?;
        Ok(graphql_output)
        
    }
    async fn burrow_devices(ctx: &Contracts, _duration: String, _devices_requested: String) -> FieldResult<String> {
        // request devices

        let ud = U256::from_dec_str(&_duration)?;
        let udr = U256::from_dec_str(&_devices_requested)?;
    
        let client_address = hex!("1F8CD3c544Bc1C225278dA0449CCD18006d93Ebe").into();  
        let tx = ctx.device_tag.
        call_with_confirmations("mintTag", (ud,udr),client_address,Options::default(), 1).await?;
        let graphql_output = serde_json::to_string(&tx)?;
        Ok(graphql_output)
    }

    async fn return_devices(ctx: &Contracts, _devices_returned: String) -> FieldResult<String> {

        // return loaned devices
       
        let client_address = hex!("1F8CD3c544Bc1C225278dA0449CCD18006d93Ebe").into();
       
        let tx = ctx.device_tag.
        call("burnTag", _devices_returned,client_address,Options::default()).await?;
        let graphql_output = serde_json::to_string(&tx)?;
        Ok(graphql_output)
    }
}




use hex_literal::hex;
use juniper::FieldResult;
use web3::{contract::Options,types::{U256, H160}};

use super::Contracts;
pub struct Querys;

#[graphql_object(context = Contracts)]
impl Querys {

    async fn devices_by_id(ctx: &Contracts, device_id: String ) -> FieldResult<String> {

        let account: H160 =  hex!("1326ead98D613c9233D905a716d8105BE029e69D").into();
        let result: U256 = ctx.device_manager.query("totalSupply",(account,device_id),account, Options::default(),None).await?;
        let graphql_output = serde_json::to_string(&result)?;
        Ok(graphql_output)
    }
    async fn tags(ctx: &Contracts, device_id: String ) -> FieldResult<String> {

        let account: H160 =  hex!("1326ead98D613c9233D905a716d8105BE029e69D").into();
        let result: Vec<U256> = ctx.device_tag.query("getTagIDs",device_id,account, Options::default(),None).await?;
        let graphql_output = serde_json::to_string(&result)?;
        Ok(graphql_output)
    }
    
}


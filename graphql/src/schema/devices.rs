use async_graphql::{Object, Context, Enum, scalar};
use hex_literal::hex;
use serde::{Serialize, Deserialize};
use web3::{contract::{Contract, Options, tokens::Tokenizable},types::{Address, U256, H160}, transports::Http, block_on};

#[derive(Default)]
pub struct DeviceQuery;

#[Object]
impl DeviceQuery {
    async fn devices(&self, ctx: &Context<'_>) -> Vec<Device>{
        let account: H160  =  hex!("d028d24f16a8893bd078259d413372ac01580769").into();
        let smart_contract = ctx.data::<Contract<Http>>().unwrap();
        let count = smart_contract.query("getDeviceCount",(), account, Options::default(),None).await.unwrap();
        let x = U256::as_u32(&count)- 1;
        let device_arr = (0..x).map( move |_|{
           let result = block_on(smart_contract.query("getStashedDevice",x ,account, Options::default(),None));
           let device: Device = result.unwrap();
           device
        })
        .collect();

        device_arr
    }
    
}

scalar!(Device);
#[derive(Serialize, Deserialize)]
struct Device {
      device_id: String,
      device_url: String,
      device_address: Address,
      device_owner: Address,
      collateral: u8,
      tag_id: u32,
      state: OwnershipState,
      loan_period: u32

}

impl Tokenizable for Device {
    fn from_token(token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
    where
        Self: Sized {
        todo!()
    }

    fn into_token(self) -> web3::ethabi::Token {
        todo!()
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq,Serialize, Deserialize)]
enum OwnershipState{
     STASHED,
     LOANED
}

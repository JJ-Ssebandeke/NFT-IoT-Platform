use async_graphql::{Object, Enum, scalar};
use hex_literal::hex;
use serde::{Serialize, Deserialize};
use web3::{contract::{Options, tokens::Tokenizable},types::{Address, U256, H160}, transports::Http, block_on};

use super::helpers::_contract;

#[derive(Default)]
pub struct DeviceQuery;

#[Object]
impl DeviceQuery {
    async fn devices(&self) -> Vec<Device> {
      
        let http = Http::new("http://localhost:7545").unwrap();
        let address = hex!("64e38Deee57C4473F3aD542859f3a84CD5DB56bD");

        let _contract = _contract(http.clone(), address.into(),include_bytes!("./abi/DeviceHub.json")).await.unwrap();

        let account: H160  =  hex!("1326ead98D613c9233D905a716d8105BE029e69D").into();
        let count = _contract.query("getDeviceCount",(), account, Options::default(),None).await.unwrap();
        let x = U256::as_u64(&count);
        let device_arr = (0..x).map( move |_|{
           let result = block_on(_contract.query("getStashedDevice",x ,account, Options::default(),None));
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
    fn from_token(_token: web3::ethabi::Token) -> Result<Self, web3::contract::Error>
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

use async_graphql::{Object, Context};
use web3::{contract::Contract,types::Address, transports::Http};

#[derive(Default)]
pub struct DeviceQuery;

#[Object]
impl DeviceQuery {
    async fn devices(&self, ctx: &Context<'_>) -> Vec<Device>{
        let smart_contract = ctx.data::<Contract<Http>>();
        let count = smart_contract.query();
        "no devices"
    }
    
}

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
enum OwnershipState{
     STASHED,
     LOANED
}

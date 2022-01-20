use hex_literal::hex;
use juniper::{RootNode, EmptySubscription, Context};
use web3::{contract::{Contract, Result}, transports::Http};

use {query::Querys, mutation::Mutations};
mod mutation;
mod query;
mod subsription;

pub type Schema = RootNode<'static, Querys, Mutations, EmptySubscription>;
pub fn build_schema() -> Schema {
 
    RootNode::new(
        Querys,
        Mutations,
        EmptySubscription::<()>::new(),
    )
}
pub struct Contracts {
    device_manager: Contract<Http>,
    device_tag: Contract<Http>

}

impl Context for Contracts { }

impl Contracts {
    fn init(&mut self, manager_abi: &[u8],tag_abi: &[u8]) -> Result<()> {

        let transport = web3::transports::Http::new("http://localhost:8545")?;
        let web3 = web3::Web3::new(transport);

        let manager_address = hex!("d028d24f16a8893bd078259d413372ac01580769").into();
        let tag_address = hex!("d048d24f16a8893bd078259d413372ac01580769").into();


        self.device_manager = Contract::from_json(web3.eth(), manager_address, manager_abi)?;
        self.device_tag = Contract::from_json(web3.eth(), tag_address, tag_abi)?;
        

        Ok(())
    }
}

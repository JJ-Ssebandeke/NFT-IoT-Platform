use web3::{contract::{Contract,Result}, transports::Http, Web3, types::H160};


pub async fn _contract(_http: Http,contract_address: H160, contract_abi: &[u8]) -> Result<Contract<Http>> {

    let web3 =  Web3::new(_http);
    let address = contract_address;
    let abi = contract_abi;
 
    let contract = Contract::from_json(web3.eth(), address , abi)?;
    Ok(contract)
}
 
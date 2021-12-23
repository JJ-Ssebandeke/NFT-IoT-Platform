mod routes;

use rocket::{build, routes};
use web3::{contract::{Contract, Result}, transports::Http, Web3};
use crate::schema;


pub async fn start() {

    let device_hub = device_contract().await;
     
    let schema = schema::build_schema()
    .data(device_hub)
    .finish();
    
    build()
        .manage(schema)
        .mount("/", routes![routes::graphql_playground])
        .launch().await;
}

async fn device_contract() -> Result<>  {

    let http = Http::new("http://localhost:8545")?;
    let web3 =  Web3::new(http);
    let address;
    let abi;
    // create a reference to the hub contract 
    let contract = Ok(Contract::from_json(web3.eth(), address , abi))?;
    contract
}

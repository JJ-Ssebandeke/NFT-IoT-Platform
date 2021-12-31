mod routes;
mod helpers;

use hex_literal::hex;
use rocket::{build, routes};
use web3::{transports::Http, Result};


use crate::schema;

use self::helpers::_contract;


pub async fn start() -> Result<()> {

    let http = Http::new("http://localhost:8545")?;
    let add = hex!("d028d24f16a8893bd078259d413372ac01580769");
    let device_hub = _contract(http, add.into(),include_bytes!("./res/SimpleStorage.abi")).await.unwrap();
    let crypto_tag = _contract(http, add.into(), include_bytes!("./res/SimpleStorage.abi")).await.unwrap();

     
    let schema = schema::build_schema()
    .data(device_hub)
    .data(crypto_tag)
    .finish();
     
    build()
        .manage(schema)
        .mount("/", routes![routes::graphql_playground])
        .launch().await;
    Ok(())
}


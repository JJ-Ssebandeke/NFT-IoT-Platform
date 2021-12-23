use async_graphql::Object;

pub struct ClientMutation;

#[Object]
impl ClientMutation{
    async fn burrow_devices(&self, username: String, password: String) -> Result<bool> {
        // request devices
    }

    async fn return_devices(&self, username: String, password: String) -> Result<bool> {
        // return loaned devices
    }

}
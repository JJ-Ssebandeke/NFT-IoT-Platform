use async_graphql::Object;

pub struct AdminMutation;

#[Object]
impl AdminMutation{
    async fn register(&self, username: String, password: String) -> Result<bool> {
        // register device
    }
}
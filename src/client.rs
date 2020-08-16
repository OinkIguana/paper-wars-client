use graphql_client::GraphQLQuery;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    server_url: String,
    authentication: Arc<RwLock<Option<String>>>,
}

impl Client {
    pub fn new(server_url: String) -> anyhow::Result<Self> {
        Ok(Client {
            client: reqwest::Client::builder()
                .user_agent("paper-wars-client")
                .build()?,
            server_url,
            authentication: Default::default(),
        })
    }

    pub async fn query<Q: GraphQLQuery>(
        &self,
        variables: Q::Variables,
    ) -> anyhow::Result<Q::ResponseData> {
        let mut request = self
            .client
            .post(&format!("{}/graphql", self.server_url))
            .json(&Q::build_query(variables));
        if let Some(token) = &*self.authentication.read().await {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        Ok(request.send().await?.json().await?)
    }
}

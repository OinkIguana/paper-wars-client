use crate::queries::*;
use graphql_client::GraphQLQuery;
use std::sync::Arc;
use async_std::sync::RwLock;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct Client {
    server_url: String,
    authentication: Arc<RwLock<Option<String>>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    server_url: String,
    authentication: Arc<RwLock<Option<String>>>,
}

impl Client {
    pub async fn resume(server_url: String, token: String) -> anyhow::Result<Self> {
        let client = Self::new(server_url)?;
        match client
            .call::<Reauthenticate>(reauthenticate::Variables { token })
            .await
        {
            Ok(response) => {
                client.set_authentication(response.authenticate).await;
            }
            Err(..) => {}
        }
        Ok(client)
    }

    pub async fn authenticate(&self, email: String, password: String) -> anyhow::Result<String> {
        let response = self
            .call::<Authenticate>(authenticate::Variables { email, password })
            .await?;
        self.set_authentication(response.authenticate.clone()).await;
        Ok(response.authenticate)
    }

    pub async fn reauthenticate(&self, token: String) -> anyhow::Result<String> {
        let response = self
            .call::<Reauthenticate>(reauthenticate::Variables { token })
            .await?;
        self.set_authentication(response.authenticate.clone()).await;
        Ok(response.authenticate)
    }

    pub async fn is_authenticated(&self) -> bool {
        self.authentication.read().await.is_some()
    }

    pub async fn set_authentication(&self, token: String) {
        *self.authentication.write().await = Some(token)
    }
}

#[cfg(not(target_arch = "wasm32"))]
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

    pub async fn call<Q: GraphQLQuery + 'static>(
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

#[cfg(target_arch = "wasm32")]
impl Client {
    pub fn new(server_url: String) -> anyhow::Result<Self> {
        Ok(Client {
            server_url,
            authentication: Default::default(),
        })
    }

    pub async fn call<Q: GraphQLQuery + 'static>(
        &self,
        variables: Q::Variables,
    ) -> anyhow::Result<Q::ResponseData> {
        let mut opts = RequestInit::new();
        opts.method("POST");
        let body = JsValue::from_serde(&Q::build_query(variables))?;
        opts.body(Some(&body));
        opts.mode(RequestMode::Cors);
        let url = format!("{}/graphql", self.server_url);
        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|value| anyhow::anyhow!("{:?}", value))?;

        let token = self.authentication.read().await;
        if let Some(token) = &*token {
            request
                .headers()
                .set("Authorization", &format!("Bearer {}", token))
                .map_err(|value| anyhow::anyhow!("{:?}", value))?;
        }

        let window = web_sys::window().unwrap();
        let response: Response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|value| anyhow::anyhow!("{:?}", value))?
            .dyn_into()
            .unwrap();
        let json = JsFuture::from(response.json().map_err(|value| anyhow::anyhow!("{:?}", value))?)
            .await
            .map_err(|value| anyhow::anyhow!("{:?}", value))?;
        Ok(json.into_serde().unwrap())
    }
}

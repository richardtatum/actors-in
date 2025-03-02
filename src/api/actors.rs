use super::client::ApiClient;
use serde::Deserialize;

pub struct ActorService {
    api_client: ApiClient,
}

#[derive(Deserialize, Debug)]
pub struct SearchPersonResult {
    pub id: u32,
    pub name: String,
    pub known_for_department: String,
}

impl ActorService {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }

    pub async fn search_actor(&self, s: &str) -> Result<Vec<SearchPersonResult>, reqwest::Error> {
        // Will need to add pagination support
        let endpoint = format!("3/search/person?query={}", s);
        return self.api_client.get::<SearchPersonResult>(&endpoint).await;
    }
}

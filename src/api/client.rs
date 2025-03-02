use reqwest::Client;
use serde::Deserialize;
use serde::de::DeserializeOwned;

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.themoviedb.org/".into(),
            api_key,
        }
    }

    // Update this to return T, when I work out how to define T is deserialisable
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<Vec<T>, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<Response<T>>()
            .await?;

        Ok(response.results)
    }
}

#[derive(Deserialize, Debug)]
#[serde(bound = "T: DeserializeOwned")]
struct Response<T: DeserializeOwned> {
    page: u32,
    results: Vec<T>,
}

use super::client::ApiClient;
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Deserialize, Debug)]
pub struct SearchPersonResult {
    pub id: i32,
    pub name: String,
    pub known_for_department: String,
    pub known_for: Vec<KnownFor>,
}

#[derive(Deserialize, Debug)]
pub struct KnownFor {
    pub id: u32,
    pub title: Option<String>, // It's 'title' for Movies, 'name' for TV
}

#[derive(Deserialize, Debug)]
pub struct GetCreditsResult {
    pub cast: Vec<KnownFor>,
}

#[derive(Deserialize, Debug)]
#[serde(bound = "T: DeserializeOwned")]
struct Response<T: DeserializeOwned> {
    page: u32,
    results: Vec<T>,
}

pub struct ActorService<'a> {
    api_client: &'a ApiClient,
}

impl<'a> ActorService<'a> {
    pub fn new(api_client: &'a ApiClient) -> Self {
        Self { api_client }
    }

    pub async fn search_actor(&self, s: &str) -> Result<Vec<SearchPersonResult>, reqwest::Error> {
        // Will need to add pagination support
        let endpoint = format!("3/search/person?query={}", s);
        let response = self
            .api_client
            .get::<Response<SearchPersonResult>>(&endpoint)
            .await?;

        return Ok(response.results);
    }

    pub async fn get_credits(&self, id: &u32) -> Result<Vec<GetCreditsResult>, reqwest::Error> {
        let endpoint = format!("3/person/{id}/movie_credits");
        let response = self
            .api_client
            .get::<Response<GetCreditsResult>>(&endpoint)
            .await?;

        return Ok(response.results);
    }
}

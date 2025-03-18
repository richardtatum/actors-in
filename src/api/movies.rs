use serde::Deserialize;

use super::client::ApiClient;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub cast: Vec<Role>,
}

#[derive(Deserialize, Debug)]
pub struct Role {
    pub id: i32,
    pub title: String,
    pub release_date: String,
    pub character: String,
}

pub struct MovieService<'a> {
    api_client: &'a ApiClient,
}

impl<'a> MovieService<'a> {
    pub fn new(api_client: &'a ApiClient) -> Self {
        Self { api_client }
    }

    pub async fn get_movies(&self, actor_id: &u32) -> Result<Vec<Role>, reqwest::Error> {
        let endpoint = format!("3/person/{actor_id}/movie_credits");
        let response = self.api_client.get::<Response>(&endpoint).await?;

        return Ok(response.cast);
    }
}

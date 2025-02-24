mod credits;
mod details;
mod traits;

use std::process::exit;

use credits::credits::CreditsResponse;
use details::details::DetailsResponse;
use reqwest::{Client, Response};
use traits::Printable;

#[tokio::main]
async fn main() {
    let movie_id = get_movie_id();
    let (details, credits) = tokio::join!(get_details(movie_id), get_credits(movie_id));

    println!("Checking for movie id: {} \n", movie_id);
    match details {
        Some(details) => {
            details.print();

            if let Some(credits) = credits {
                credits.print();
            }
        }
        None => {
            eprintln!("No movie found with this id.");
            return;
        }
    }
}

async fn get_details(movie_id: u32) -> Option<DetailsResponse> {
    let url = format!("https://api.themoviedb.org/3/movie/{movie_id}");
    let bearer_token = get_bearer_token();
    let resp = request(url, bearer_token).await?;

    return resp.json::<DetailsResponse>().await.ok();
}

async fn get_credits(movie_id: u32) -> Option<CreditsResponse> {
    let url = format!("https://api.themoviedb.org/3/movie/{movie_id}/credits");
    let bearer_token = get_bearer_token();
    let resp = request(url, bearer_token).await?;

    return resp.json::<CreditsResponse>().await.ok();
}

async fn request(url: String, bearer_token: String) -> Option<Response> {
    return Client::builder()
        .build()
        .unwrap()
        .get(url)
        .bearer_auth(bearer_token)
        .send()
        .await
        .ok();
}

fn get_bearer_token() -> String {
    let variable_name = "ACCESS_TOKEN";
    return std::env::var(variable_name).unwrap_or_else(|_| {
        eprintln!("'{}' environment variable not set", variable_name);
        exit(1)
    });
}

fn get_movie_id() -> u32 {
    return std::env::args()
        .nth(1)
        .ok_or("Must provide a movie id!")
        .and_then(|id| {
            id.parse::<u32>()
                .map_err(|_| "Invalid movie id. Must be a number!")
        })
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1)
        });
}

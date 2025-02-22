use std::process::exit;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MovieResponse {
    title: String,
    release_date: String,
    runtime: u32,
    overview: String,
}

fn main() {
    let movie_id = std::env::args()
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

    println!("Checking for movie id: {} \n", movie_id);
    match request_movie(movie_id) {
        Some(movie) => {
            println!("Title: {}", movie.title);
            println!("Release Date: {}", movie.release_date);
            println!("Runtime: {}", movie.runtime);
            println!("Overview: {}", movie.overview);
        }
        None => {
            eprintln!("No movie found with this id.");
            return;
        }
    }
}

#[tokio::main]
async fn request_movie(movie_id: u32) -> Option<MovieResponse> {
    let token = std::env::var("BEARER_TOKEN").unwrap_or_else(|_| {
        eprintln!("BEARER_TOKEN environment variable not set");
        exit(1)
    });

    let url = format!("https://api.themoviedb.org/3/movie/{movie_id}");
    let resp = Client::builder()
        .build()
        .unwrap()
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    return resp.json::<MovieResponse>().await.ok();
}

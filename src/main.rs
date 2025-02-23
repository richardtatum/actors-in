use std::process::exit;

use reqwest::{Client, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MovieResponse {
    title: String,
    release_date: String,
    runtime: u32,
    overview: String,
}

#[derive(Deserialize, Debug)]
struct CreditsResponse {
    cast: Vec<Cast>,
    crew: Vec<Crew>,
}

#[derive(Deserialize, Debug)]
struct Cast {
    name: String,
    character: String,
}

#[derive(Deserialize, Debug)]
struct Crew {
    name: String,
    job: String,
}

#[tokio::main]
async fn main() {
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

    let (movie, credits) = tokio::join!(get_movie(movie_id), get_credits(movie_id));

    println!("Checking for movie id: {} \n", movie_id);
    match movie {
        Some(movie) => {
            println!("Title: {}", movie.title);
            println!("Release Date: {}", movie.release_date);
            println!("Runtime: {}", movie.runtime);
            println!("Overview: {}", movie.overview);

            if let Some(credits) = credits {
                println!("\nCast:");
                for cast in credits.cast.iter().take(10) {
                    println!("{} - {}", cast.name, cast.character);
                }

                println!("\nCrew:");
                for crew in credits.crew.iter().take(5) {
                    println!("{} - {}", crew.name, crew.job);
                }
            }
        }
        None => {
            eprintln!("No movie found with this id.");
            return;
        }
    }
}

async fn get_movie(movie_id: u32) -> Option<MovieResponse> {
    let url = format!("https://api.themoviedb.org/3/movie/{movie_id}");
    let bearer_token = get_bearer_token();
    let resp = request(url, bearer_token).await?;

    return resp.json::<MovieResponse>().await.ok();
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
    return std::env::var("BEARER_TOKEN").unwrap_or_else(|_| {
        eprintln!("BEARER_TOKEN environment variable not set");
        exit(1)
    });
}

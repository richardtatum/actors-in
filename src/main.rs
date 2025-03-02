mod api;
mod credits;
mod details;
mod traits;

use std::process::exit;

use api::{actors::ActorService, client::ApiClient};
use credits::credits::CreditsResponse;
use details::details::DetailsResponse;
use reqwest::{Client, Response};

#[tokio::main]
async fn main() {
    let api_key = get_access_token();
    let api_client = ApiClient::new(api_key);
    let actor_service = ActorService::new(api_client);
    let resp = actor_service.search_actor("tom ford".into()).await;
    match resp {
        Ok(result) => {
            result.iter().for_each(|person| {
                println!("Id: {}", person.id);
                println!("Name: {}", person.name);
                println!("Known for: {}", person.known_for_department);
                println!()
            });
        }
        Err(_) => println!("Something went wrong!"),
    }
}

fn get_access_token() -> String {
    let variable_name = "ACCESS_TOKEN";
    return std::env::var(variable_name).unwrap_or_else(|_| {
        eprintln!("'{}' environment variable not set", variable_name);
        exit(1)
    });
}

// fn get_movie_id() -> u32 {
//     return std::env::args()
//         .nth(1)
//         .ok_or("Must provide a movie id!")
//         .and_then(|id| {
//             id.parse::<u32>()
//                 .map_err(|_| "Invalid movie id. Must be a number!")
//         })
//         .unwrap_or_else(|e| {
//             eprintln!("{}", e);
//             exit(1)
//         });
}

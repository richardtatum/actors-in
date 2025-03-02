mod api;
mod details;
mod traits;

use std::process::exit;

use api::{actors::ActorService, client::ApiClient};

#[tokio::main]
async fn main() {
    let name = get_first_arg();
    let api_key = get_access_token();
    let api_client = ApiClient::new(api_key);
    let actor_service = ActorService::new(api_client);
    let resp = actor_service.search_actor(&name).await;
    match resp {
        Ok(result) => {
            result.iter().for_each(|person| {
                let movies = person
                    .known_for
                    .iter()
                    .filter_map(|movie| movie.title.as_deref())
                    .collect::<Vec<&str>>()
                    .join(", ");

                println!("Id: {}", person.id);
                println!("Name: {}", person.name);
                println!("Known for: {}", person.known_for_department);
                println!("Movies: {}", movies);
                println!();
            });
        }
        Err(e) => println!("Something went wrong! {}", e),
    }
}

fn get_access_token() -> String {
    let variable_name = "ACCESS_TOKEN";
    return std::env::var(variable_name).unwrap_or_else(|_| {
        eprintln!("'{}' environment variable not set", variable_name);
        exit(1)
    });
}

fn get_first_arg() -> String {
    return std::env::args()
        .nth(1)
        .ok_or("Must provide a name!")
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1)
        });
}

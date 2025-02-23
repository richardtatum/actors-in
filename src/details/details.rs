use crate::traits::Printable;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DetailsResponse {
    title: String,
    release_date: String,
    runtime: u32,
    overview: String,
}

impl Printable for DetailsResponse {
    fn print(&self) {
        println!("Title: {}", self.title);
        println!("Release Date: {}", self.release_date);
        println!("Runtime: {}", self.runtime);
        println!("Overview: {}", self.overview);
    }
}

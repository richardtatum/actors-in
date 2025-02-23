use crate::traits::Printable;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(super) struct Crew {
    name: String,
    job: String,
}

impl Printable for Crew {
    fn print(&self) {
        println!("{} - {}", self.name, self.job)
    }
}

use crate::traits::Printable;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(super) struct Cast {
    name: String,
    character: String,
}

impl Printable for Cast {
    fn print(&self) {
        println!("{} - {}", self.name, self.character)
    }
}

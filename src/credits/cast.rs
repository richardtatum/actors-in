use crate::traits::Printable;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
// pub(super) makes this only available within the crate/module
pub(super) struct Cast {
    name: String,
    character: String,
}

impl Printable for Cast {
    fn print(&self) {
        println!("{} - {}", self.name, self.character)
    }
}

use crate::credits::cast::Cast;
use crate::credits::crew::Crew;
use crate::traits::Printable;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreditsResponse {
    cast: Vec<Cast>,
    crew: Vec<Crew>,
}

impl Printable for CreditsResponse {
    fn print(&self) {
        let max_cast = 5;
        let max_crew = 3;

        println!("\nCast:");
        for person in self.cast.iter().take(max_cast) {
            person.print();
        }

        println!("\nCrew:");
        for person in self.crew.iter().take(max_crew) {
            person.print();
        }
    }
}

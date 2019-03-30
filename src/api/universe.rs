use reqwest;
use shared::{Id, Description, Universe};
use crate::env::SERVER_URL;

pub fn list_universes() -> Result<Vec<Description<Universe>>, String> {
    get!("{}/universe", &*SERVER_URL)
}

pub fn load_universe(id: Id<Universe>) -> Result<Universe, String> {
    get!("{}/universe/{}", &*SERVER_URL, id)
}

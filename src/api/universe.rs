use futures::Future;
use shared::{Id, Description, Universe};
use crate::env::env;

pub fn list_universes() -> impl Future<Item = Vec<Description<Universe>>, Error = String> {
    get!("{}/universe", env("SERVER_URL"))
}

pub fn load_universe(id: Id<Universe>) -> impl Future<Item = Universe, Error = String> {
    get!("{}/universe/{}", env("SERVER_URL"), id)
}

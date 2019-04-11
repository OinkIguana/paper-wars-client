use futures::Future;
use shared::{api, Game, Id, Universe, MapType};
use crate::env::env;

pub fn new_game(name: String, universe: Id<Universe>, map: Id<MapType>) -> impl Future<Item = Game, Error = String> {
    let request = api::NewGame {
        name,
        universe,
        map,
    };
    post!(request => "{}/game/new", env("SERVER_URL"))
}

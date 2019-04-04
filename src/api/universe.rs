use futures::Future;
use shared::{Id, Description, Universe};
use crate::env::env;

pub fn load_localization() -> impl Future<Item = (), Error = String> {
    get!("{}/l10n", env("SERVER_URL"))
        .map(|ftl: String| crate::localization::set_source(ftl))
}

pub fn list_universes() -> impl Future<Item = Vec<Description<Universe>>, Error = String> {
    get!("{}/universe", env("SERVER_URL"))
}

pub fn load_universe(id: Id<Universe>) -> impl Future<Item = Universe, Error = String> {
    get!("{}/universe/{}", env("SERVER_URL"), id)
}

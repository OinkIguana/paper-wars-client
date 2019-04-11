use futures::{Future, IntoFuture};
use serde::{Serialize, Deserialize};
use shared::crypto::Signed;

#[cfg(feature="native")]
pub fn sign<T: Serialize>(data: T) -> impl Future<Item = Signed<T>, Error = ()> {
    Signed::sign(data).into_future()
}

#[cfg(feature="native")]
pub fn verify<T: for<'de> Deserialize<'de>>(data: Signed<T>) -> impl Future<Item = T, Error = ()> {
    data.verify().into_future()
}

// TODO: WASM will have to reach out to web_sys::Crypto and use Serde to build the Signed blobs

#[cfg(feature="wasm")]
pub fn sign<T: Serialize>(data: T) -> impl Future<Item = Signed<T>, Error = ()> {
    Signed::sign(data).into_future()
}

#[cfg(feature="wasm")]
pub fn verify<T: for<'de> Deserialize<'de>>(data: Signed<T>) -> impl Future<Item = T, Error = ()> {
    data.verify().into_future()
}

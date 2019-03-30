use lazy_static::lazy_static;

#[cfg(features = "wasm")]      use crate::wasm::env;
#[cfg(not(features = "wasm"))] use std::env::var as env;

lazy_static! {
    pub static ref SERVER_URL: String = env("SERVER_URL").unwrap();
}

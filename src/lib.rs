#[macro_use]
extern crate rental;

mod api;
mod env;
mod localization;

#[cfg(feature="wasm")] mod wasm;
#[cfg(feature="wasm")] pub use wasm::*;

#[cfg(feature="native")] pub use api::*;
#[cfg(feature="native")] pub use localization::*;

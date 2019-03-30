macro_rules! get {
    ($($path:tt)*) => {
        reqwest::get(&format!($($path)*))
            .map_err(|error| format!("API Error (f{} l{} c{}): {}", file!(), line!(), column!(), error))
            .and_then(|read| serde_cbor::from_reader(read)
                .map_err(|error| format!("API Error (f{} l{} c{}): {}", file!(), line!(), column!(), error))
            )
    }
}

mod universe;
pub use universe::*;

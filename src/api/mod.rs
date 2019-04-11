#[cfg(feature="native")]
lazy_static::lazy_static! {
    static ref CLIENT: reqwest::r#async::Client = reqwest::r#async::Client::new();
}

#[cfg(feature="native")]
macro_rules! post {
    ($($path:expr),+) => (post!(@_POST crate::api::CLIENT.post(&format!($($path),+))
        .build()
        .unwrap()));
    ($body:expr => $($path:expr),+) => (post!(@_POST crate::api::CLIENT.post(&format!($($path),+))
        .body(serde_cbor::to_vec(&$body).unwrap())
        .build()
        .unwrap()));

    (@_POST $request:expr) => {{
        use futures::{Future as _, Stream as _};
        // TODO: pass accept language header
        crate::api::CLIENT.execute($request)
            .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            .map(|response| response.into_body()
                .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            )
            .flatten_stream()
            .concat2()
            .and_then(|vec| serde_cbor::from_slice(&vec)
                .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            )
    }}
}

#[cfg(feature="native")]
macro_rules! get {
    ($($path:expr),+) => {{
        use futures::{Future as _, Stream as _};
        let request = crate::api::CLIENT.get(&format!($($path),+)).build().unwrap();
        // TODO: pass accept language header
        crate::api::CLIENT.execute(request)
            .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            .map(|response| response.into_body()
                .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            )
            .flatten_stream()
            .concat2()
            .and_then(|vec| serde_cbor::from_slice(&vec)
                .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            )
    }}
}

#[cfg(feature="wasm")]
macro_rules! post {
    ($($path:expr),+) => (post!(@_POST None => $($path),+));
    ($body:expr => $($path:expr),+) => (post!(@_POST Some(&JsValue::from_serde(&serde_cbor::to_vec(&$body).unwrap()).unwrap()) => $($path),+));

    (@_POST $body:expr => $($path:expr),+) => {{
        use futures::Future as _;
        use wasm_bindgen::{JsCast, JsValue};
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, Response, RequestInit, window};
        use js_sys::{ArrayBuffer, Uint8Array};
        let mut opts = RequestInit::new();
        opts.body($body);
        opts.method("POST");
        let request = Request::new_with_str_and_init(&format!($($path),*), &opts).unwrap();
        let promise = window().unwrap().fetch_with_request(&request);
        JsFuture::from(promise)
            .and_then(|response| {
                let response: Response = response.dyn_into().unwrap();
                response.array_buffer()
            })
            .and_then(JsFuture::from)
            .map(|body| -> Vec<u8> {
                let buffer: ArrayBuffer = body.dyn_into().unwrap();
                let array = Uint8Array::new(&buffer);
                let mut vec = vec![0u8; array.byte_length() as usize];
                array.copy_to(&mut vec);
                vec
            })
            .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            .and_then(|vec| serde_cbor::from_slice(&vec)
                .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            )
    }}
}

#[cfg(feature="wasm")]
macro_rules! get {
    ($($path:expr),+) => {{
        use futures::Future as _;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, Response, RequestInit, window};
        use js_sys::{ArrayBuffer, Uint8Array};
        let mut opts = RequestInit::new();
        opts.method("GET");
        let request = Request::new_with_str_and_init(&format!($($path),*), &opts).unwrap();
        let promise = window().unwrap().fetch_with_request(&request);
        JsFuture::from(promise)
            .and_then(|response| {
                let response: Response = response.dyn_into().unwrap();
                response.array_buffer()
            })
            .and_then(JsFuture::from)
            .map(|body| -> Vec<u8> {
                let buffer: ArrayBuffer = body.dyn_into().unwrap();
                let array = Uint8Array::new(&buffer);
                let mut vec = vec![0u8; array.byte_length() as usize];
                array.copy_to(&mut vec);
                vec
            })
            .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            .and_then(|vec| serde_cbor::from_slice(&vec)
                .map_err(|error| format!("API Error ([{}]:{}#{}): {:?}", file!(), line!(), column!(), error))
            )
    }}
}

mod game;
mod universe;
pub use game::*;
pub use universe::*;

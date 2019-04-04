#![allow(non_camel_case_types)]
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use serde::{Serialize, Deserialize};
use js_sys::Promise;
use futures::Future;

// simple framework to support environment variables in WASM context

struct Env {
    server_url: Option<String>,
}

static mut ENV: Env = Env { server_url: None };

pub(crate) fn env(name: &str) -> Option<&String> {
    match name {
        "SERVER_URL" => unsafe { ENV.server_url.as_ref() }
        _ => None,
    }
}

#[wasm_bindgen]
pub fn register_env(name: &str, value: &str) {
    match name {
        "SERVER_URL" => unsafe { ENV.server_url = Some(value.to_owned()); }
        _ => {}
    }
}

macro_rules! wrap {
    (struct $w:ident($t:ty)) => {
        #[wasm_bindgen]
        #[derive(Serialize, Deserialize)]
        pub struct $w($t);

        impl From<$t> for $w {
            fn from(t: $t) -> Self {
                $w(t)
            }
        }

        impl From<$w> for $t {
            fn from(w: $w) -> Self {
                w.0
            }
        }

        impl AsRef<$t> for $w {
            fn as_ref(&self) -> &$t {
                &self.0
            }
        }

        impl AsMut<$t> for $w {
            fn as_mut(&mut self) -> &mut $t {
                &mut self.0
            }
        }
    };

    (async fn $name:ident($($arg:ident: $type:ty),*), $fn:path) => {
        #[wasm_bindgen]
        pub fn $name($($arg: JsValue),*) -> Promise {
            let future = $fn($($arg.into_serde::<$type>().unwrap().into()),*)
                .map(|value| JsValue::from_serde(&value).unwrap())
                .map_err(|error| JsValue::from_serde(&error).unwrap());
            future_to_promise(future)
        }
    };

    (fn $name:ident($($arg:ident: $type:ty),*), $fn:path) => {
        #[wasm_bindgen]
        pub fn $name($($arg: JsValue),*) -> JsValue {
            JsValue::from_serde(&$fn($($arg.into_serde().unwrap()),*)).unwrap()
        }
    };
}

wrap!(struct I18n(shared::I18n));
wrap!(struct DamageType(shared::DamageType));
wrap!(struct MapType(shared::MapType));
wrap!(struct ModifierClass(shared::ModifierClass));
wrap!(struct ModifierType(shared::ModifierType));
wrap!(struct Race(shared::Race));
wrap!(struct Research(shared::Research));
wrap!(struct Resource(shared::Resource));
wrap!(struct Stat(shared::Stat));
wrap!(struct TileType(shared::TileType));
wrap!(struct Universe(shared::Universe));
wrap!(struct Description_Universe(shared::Description<shared::Universe>));
wrap!(struct Id_Universe(shared::Id<shared::Universe>));
wrap!(struct UnitClass(shared::UnitClass));
wrap!(struct UnitType(shared::UnitType));

wrap!(async fn load_localization(), crate::api::load_localization);
wrap!(async fn list_universes(), crate::api::list_universes);
wrap!(async fn load_universe(id: Id_Universe), crate::api::load_universe);
#[wasm_bindgen]
pub fn set_locales(language: String) {
    crate::localization::set_locales(&language.split(",").collect::<Vec<_>>())
}
wrap!(fn localize(language: I18n), crate::localization::localize);

#![allow(non_camel_case_types)]
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;
use futures::Future;

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
    (type $w:ident, $t:ty) => {
        #[wasm_bindgen]
        pub struct $w(Box<$t>);
        impl From<$t> for $w {
            fn from(t: $t) -> Self {
                $w(Box::new(t))
            }
        }

        impl From<$w> for $t {
            fn from(w: $w) -> Self {
                *w.0
            }
        }

        impl AsRef<$t> for $w {
            fn as_ref(&self) -> &$t {
                self.0.as_ref()
            }
        }

        impl AsMut<$t> for $w {
            fn as_mut(&mut self) -> &mut $t {
                self.0.as_mut()
            }
        }
    };

    (fn $name:ident($($arg:ident: $type:ty),*), $fn:path) => {
        #[wasm_bindgen]
        pub fn $name($($arg: $type),*) -> Promise {
            let future = $fn($($arg.into()),*)
                .map(|value| JsValue::from_serde(&value).unwrap())
                .map_err(|error| JsValue::from_serde(&error).unwrap());
            future_to_promise(future)
        }
    }
}

wrap!(type I18n, shared::I18n);
wrap!(type DamageType, shared::DamageType);
wrap!(type MapType, shared::MapType);
wrap!(type ModifierClass, shared::ModifierClass);
wrap!(type ModifierType, shared::ModifierType);
wrap!(type Race, shared::Race);
wrap!(type Research, shared::Research);
wrap!(type Resource, shared::Resource);
wrap!(type Stat, shared::Stat);
wrap!(type TileType, shared::TileType);
wrap!(type Id_Universe, shared::Id<shared::Universe>);
wrap!(type Universe, shared::Universe);
wrap!(type Description_Universe, shared::Description<shared::Universe>);
wrap!(type UnitClass, shared::UnitClass);
wrap!(type UnitType, shared::UnitType);

wrap!(fn list_universes(), crate::api::list_universes);
wrap!(fn load_universe(id: Id_Universe), crate::api::load_universe);

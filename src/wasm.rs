#![allow(non_camel_case_types)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) fn env(var: &str) -> Option<String>;
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

#[wasm_bindgen]
pub fn list_universes() -> Result<Vec<Description_Universe>, JsValue> {
    crate::api::list_universes()
        .map(|vec| vec.into_iter().map(Into::into).collect())
        .map_err(|err| JsValue::from_serde(&err).unwrap())
}

#[wasm_bindgen]
pub fn load_universe(id: Id_Universe) -> Result<Universe, JsValue> {
    crate::api::load_universe(id.into())
        .map(Into::into)
        .map_err(|err| JsValue::from_serde(&err).unwrap())
}

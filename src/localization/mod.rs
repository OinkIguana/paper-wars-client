use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;
use shared::I18n;

rental! {
    pub mod lion {
        #[rental]
        pub struct Lion {
            resource: Box<fluent_bundle::FluentResource>,
            bundle: fluent_bundle::FluentBundle<'resource>,
        }
    }
}

lazy_static! {
    static ref LANGUAGE: Arc<Mutex<Option<Vec<String>>>> = Arc::default();
    static ref LION: Arc<Mutex<Option<lion::Lion>>> = Arc::default();
}

pub fn set_locales(locales: &[&str]) {
    *LANGUAGE.lock().unwrap() = Some(locales.into_iter().map(|s| s.to_string()).collect());
}

pub fn get_locales() -> Vec<String> {
    LANGUAGE.lock().unwrap().clone().unwrap_or(vec![])
}

pub fn set_source(source: String) {
    let resource = match fluent_bundle::FluentResource::try_new(source) {
        Ok(resource) => resource,
        Err((resource, _)) => resource,
    };
    let lion = lion::Lion::new(Box::new(resource), |resource| {
        let mut bundle = fluent_bundle::FluentBundle::new(&get_locales());
        bundle.add_resource(resource).ok();
        bundle
    });
    *LION.lock().unwrap() = Some(lion);
}

pub fn localize(key: I18n) -> String {
    let lion = LION.lock().unwrap();
    lion.as_ref()
        .and_then(|lion| lion.rent(|bundle| bundle.format(key.as_ref(), None)))
        .map(|(formatted, _)| formatted)
        .unwrap_or(key.as_ref().to_string())
}

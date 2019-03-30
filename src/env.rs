#[cfg(feature="wasm")]
pub(crate) fn env(name: &str) -> &String {
    crate::wasm::env(name).unwrap()
}

#[cfg(feature="native")]
pub(crate) fn env(name: &str) -> String {
    std::env::var(name).unwrap()
}

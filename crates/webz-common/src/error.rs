use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid network string given: {0}")]
    InvalidNetwork(String),
    #[error("Invalid amount: {0}")]
    InvalidAmount(#[from] zcash_primitives::transaction::components::amount::BalanceError),
}

impl From<Error> for JsValue {
    fn from(e: Error) -> Self {
        js_sys::Error::new(&e.to_string()).into()
    }
}

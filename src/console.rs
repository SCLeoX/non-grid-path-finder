use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn info(s: &str);
}

#[macro_export]
macro_rules! console_info {
    ($($args:tt)*) => (crate::console::info(&format_args!($($args)*).to_string()))
}

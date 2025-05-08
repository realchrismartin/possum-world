use web_sys::console;
use wasm_bindgen::JsValue;

pub fn log_value<T: wasm_bindgen::JsCast>(value: &T)
{
    console::log_1(&value.into());
}

pub fn log(value : &str)
{
    console::log_1(&JsValue::from_str(value).into());
}

pub fn log_string(value: &String)
{
    log(value.as_str());
}
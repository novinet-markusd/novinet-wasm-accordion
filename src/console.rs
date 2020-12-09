use wasm_bindgen::prelude::*;

const ERR_DEFAULT: &str = "Something went wrong.";
const ERR_ACCORDION: &str = "It seems like there is an error in your accordion syntax.";
const ERR_EVENT_LISTENER: &str = "Could not add event listener.";
const ERR_CONVERSION: &str = "Could not convert.";
const ERR_REQUEST: &str = "Error, while requsting or recieving data.";

pub fn error(code: i32) {
    let msg = match code {
        0 => ERR_DEFAULT,
        1 => ERR_ACCORDION,
        2 => ERR_EVENT_LISTENER,
        3 => ERR_CONVERSION,
        4 => ERR_REQUEST,
        _ => ERR_DEFAULT,
    };

    let error = JsValue::from(msg);
    web_sys::console::error_1(&error);
}

pub fn _log(msg: &str) {
    let msg = JsValue::from(msg);
    web_sys::console::log_1(&msg);
}

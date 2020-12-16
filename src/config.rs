use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Config {
    pub class_item_open: &'static str,
    pub class_single: &'static str,
    pub sel_accordion: &'static str,
    pub sel_item: &'static str,
    pub sel_wrapper: &'static str,
}

impl Config {
    pub fn default() -> Config {
        Config {
            class_item_open: "nv-open",
            class_single: "nv-single",
            sel_accordion: ".nv-accordion",
            sel_item: ".nv-accordion-item",
            sel_wrapper: ".nv-wrapper",
        }
    }
}

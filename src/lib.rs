use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod animation;
mod config;
mod console;
mod dom;

use config::Config;

// set the error hook
#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// call this and next function when dom ready
// so all the event listeners can be set
#[wasm_bindgen]
pub fn accordion_default() -> Option<bool> {
    let config = Config::default();
    init(config)
}

#[wasm_bindgen]
pub fn accordion(conf: JsValue) -> Option<bool> {
    let conf = Config::from(conf);
    init(conf)
}

fn init(config: Config) -> Option<bool> {
    dom::window_add_event_listener("resize", |_| {
        let sel = format!(
            "{} {}.{}",
            config.sel_accordion, config.sel_item, config.class_item_open
        );
        dom::select_all_then_foreach(&sel, |item: &web_sys::Element| {
            open_item(item, &config);
        });
    });

    dom::select_all_then_foreach(config.sel_accordion, |acc: &web_sys::Element| {
        if acc.class_list().contains(config.class_single) {
            prepare_single_accordion(acc, &config);
        }
    })?;

    dom::select_all_then_foreach(
        &format!("{} {}", config.sel_accordion, ".nv-header"),
        |header: &web_sys::Element| {
            dom::add_mouse_event_listener(header, "click", |event: web_sys::MouseEvent| {
                if let Some(item) = find_item(event, &config) {
                    on_click(&item, &config);
                } else {
                    console::error(1);
                }
            });
        },
    )
}

fn on_click(item: &web_sys::Element, config: &Config) {
    if let Some(acc) = find_accordion(item, config) {
        if acc.class_list().contains(config.class_single) {
            if !item.class_list().contains(config.class_item_open) {
                close_all_but(&acc, item, config);
                open_item(item, config);
            }
        } else {
            if item.class_list().contains(config.class_item_open) {
                close_item(item, config);
            } else {
                open_item(item, config);
            }
        }
    } else {
        console::error(1);
    }
}

fn open_item(item: &web_sys::Element, config: &Config) {
    dom::add_to_class_list(item, config.class_item_open);
    animation::animation(item, animation::AnimationDirection::Open);
}

fn close_item(item: &web_sys::Element, config: &Config) {
    dom::remove_from_class_list(item, config.class_item_open);
    animation::animation(item, animation::AnimationDirection::Close);
}

fn find_item(event: web_sys::MouseEvent, config: &Config) -> Option<web_sys::Element> {
    let target = event.target()?;
    let target = target.dyn_ref::<web_sys::Element>()?;

    if let Ok(ele) = target.closest(config.sel_item) {
        ele
    } else {
        None
    }
}

fn find_accordion(item: &web_sys::Element, config: &Config) -> Option<web_sys::Element> {
    if let Ok(closest) = item.closest(config.sel_accordion) {
        closest
    } else {
        None
    }
}

fn prepare_single_accordion(acc: &web_sys::Element, config: &Config) {
    let sel_open = format!("{}.{}", config.sel_item, config.class_item_open);

    if let Ok(nodes) = acc.query_selector_all(&sel_open) {
        match nodes.length() {
            0 => {
                if let Ok(item) = acc.query_selector(config.sel_item) {
                    if let Some(item) = item {
                        open_item(&item, config);
                    }
                }
            }
            1 => {}
            _ => console::error(1),
        }
    }
}

fn close_all_but(acc: &web_sys::Element, item: &web_sys::Element, config: &Config) {
    let sel = format!("{}.{}", config.sel_item, config.class_item_open);
    if let Ok(nodes) = acc.query_selector_all(&sel) {
        for i in 0..nodes.length() {
            if let Some(other_item) = nodes.item(i) {
                if let Some(other_item) = other_item.dyn_ref::<web_sys::Element>() {
                    if other_item != item {
                        close_item(other_item, config);
                    }
                }
            }
        }
    }
}

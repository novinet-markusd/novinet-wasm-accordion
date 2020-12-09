use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod animation;
mod console;
mod dom;

const CLASS_ITEM_OPEN: &str = "nv-open";
const CLASS_SINGLE: &str = "nv-single";
const SEL_ACCORDION: &str = ".nv-accordion";
const SEL_ITEM: &str = ".nv-accordion-item";
const SEL_WRAPPER: &str = ".nv-wrapper";

#[wasm_bindgen]
pub fn add_event_listeners_accordion() -> Option<bool> {
    dom::window_add_event_listener("resize", |_| {
        let sel = format!("{} {}.{}", SEL_ACCORDION, SEL_ITEM, CLASS_ITEM_OPEN);
        dom::select_all_then_foreach(&sel, |item: &web_sys::Element| {
            open_item(item);
        });
    });

    dom::select_all_then_foreach(SEL_ACCORDION, |acc: &web_sys::Element| {
        if acc.class_list().contains(CLASS_SINGLE) {
            prepare_single_accordion(acc);
        }
    })?;

    dom::select_all_then_foreach(
        &format!("{} {}", SEL_ACCORDION, ".nv-header"),
        |header: &web_sys::Element| {
            dom::add_mouse_event_listener(header, "click", |event: web_sys::MouseEvent| {
                if let Some(item) = find_item(event) {
                    on_click(&item);
                } else {
                    console::error(1);
                }
            });
        },
    )
}

fn on_click(item: &web_sys::Element) {
    if let Some(acc) = find_accordion(item) {
        if acc.class_list().contains(CLASS_SINGLE) {
            if !item.class_list().contains(CLASS_ITEM_OPEN) {
                close_all_but(&acc, item);
                open_item(item);
            }
        } else {
            if item.class_list().contains(CLASS_ITEM_OPEN) {
                close_item(item);
            } else {
                open_item(item);
            }
        }
    } else {
        console::error(1);
    }
}

fn open_item(item: &web_sys::Element) {
    dom::add_to_class_list(item, CLASS_ITEM_OPEN);
    animation::animation(item, animation::AnimationDirection::Open);
}

fn close_item(item: &web_sys::Element) {
    dom::remove_from_class_list(item, CLASS_ITEM_OPEN);
    animation::animation(item, animation::AnimationDirection::Close);
}

fn find_item(event: web_sys::MouseEvent) -> Option<web_sys::Element> {
    let target = event.target()?;
    let target = target.dyn_ref::<web_sys::Element>()?;

    if let Ok(ele) = target.closest(SEL_ITEM) {
        ele
    } else {
        None
    }
}

fn find_accordion(item: &web_sys::Element) -> Option<web_sys::Element> {
    if let Ok(closest) = item.closest(SEL_ACCORDION) {
        closest
    } else {
        None
    }
}

fn prepare_single_accordion(acc: &web_sys::Element) {
    let sel_open = format!("{}.{}", SEL_ITEM, CLASS_ITEM_OPEN);

    if let Ok(nodes) = acc.query_selector_all(&sel_open) {
        match nodes.length() {
            0 => {
                if let Ok(item) = acc.query_selector(SEL_ITEM) {
                    if let Some(item) = item {
                        open_item(&item);
                    }
                }
            }
            1 => {}
            _ => console::error(1),
        }
    }
}

fn close_all_but(acc: &web_sys::Element, item: &web_sys::Element) {
    let sel = format!("{}.{}", SEL_ITEM, CLASS_ITEM_OPEN);
    if let Ok(nodes) = acc.query_selector_all(&sel) {
        for i in 0..nodes.length() {
            if let Some(other_item) = nodes.item(i) {
                if let Some(other_item) = other_item.dyn_ref::<web_sys::Element>() {
                    if other_item != item {
                        close_item(other_item);
                    }
                }
            }
        }
    }
}

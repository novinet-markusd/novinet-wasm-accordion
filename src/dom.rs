use crate::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn window() -> Option<web_sys::Window> {
    if let Some(win) = web_sys::window() {
        Some(win)
    } else {
        console::error(0);
        None
    }
}

pub fn window_add_event_listener<T>(event_name: &str, handler: T)
where
    T: 'static + FnMut(web_sys::Event),
{
    if let Some(win) = window() {
        let cb: wasm_bindgen::prelude::Closure<dyn std::ops::FnMut(_)> =
            Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        if let Err(_) =
            win.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
        {
            console::error(2);
        }
        cb.forget();
    }
}

pub fn add_mouse_event_listener<T>(ele: &web_sys::Element, event_name: &str, handler: T)
where
    T: 'static + FnMut(web_sys::MouseEvent),
{
    let cb = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    if let Err(_) = ele.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref()) {
        console::error(2);
    }
    cb.forget();
}

pub fn request_animation_frame(handler: &Closure<dyn FnMut()>) {
    if let Some(win) = window() {
        if let Err(_) = win.request_animation_frame(handler.as_ref().unchecked_ref()) {
            console::error(0);
        }
    }
}

pub fn select_all_then_foreach<T>(sel: &str, callback: T) -> Option<bool>
where
    T: Fn(&web_sys::Element),
{
    let window = web_sys::window()?;
    let document = window.document()?;

    let nodes = document.query_selector_all(sel);
    foreach_nodes(nodes, callback)
}

fn foreach_nodes<T>(nodes: Result<web_sys::NodeList, JsValue>, callback: T) -> Option<bool>
where
    T: Fn(&web_sys::Element),
{
    if let Ok(nodes) = nodes {
        if nodes.length() == 0 {
            return Some(false);
        }

        for i in 0..nodes.length() {
            if let Some(item) = nodes.item(i) {
                if let Some(item) = item.dyn_ref::<web_sys::Element>() {
                    callback(item);
                }
            }
        }

        Some(true)
    } else {
        Some(false)
    }
}

pub fn add_to_class_list(ele: &web_sys::Element, class: &str) -> Option<()> {
    let class_list = ele.class_list();

    if let Ok(_) = class_list.add_1(class) {
        Some(())
    } else {
        None
    }
}

pub fn remove_from_class_list(ele: &web_sys::Element, class: &str) -> Option<()> {
    let class_list = ele.class_list();

    if let Ok(_) = class_list.remove_1(class) {
        Some(())
    } else {
        None
    }
}

pub fn set_style_height(ele: &web_sys::Element, val: f32) {
    let ele = ele.dyn_ref::<web_sys::HtmlElement>();
    if let Some(ele) = ele {
        let style = ele.style();
        let val = format!("{}px", val);
        if let Err(_) = style.set_property("height", &val) {
            console::error(0);
        }
    } else {
        console::error(3);
    }
}

pub fn get_style_height(ele: &web_sys::Element) -> Option<f32> {
    let ele = ele.dyn_ref::<web_sys::HtmlElement>();
    if let Some(ele) = ele {
        let style = ele.style();
        if let Ok(val) = style.get_property_value("height") {
            let mut val = val.split("px");
            if let Some(s) = val.next() {
                if let Ok(s) = s.parse::<f32>() {
                    return Some(s);
                }
            }
        }
        None
    } else {
        console::error(3);
        None
    }
}

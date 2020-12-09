use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::console;
use crate::dom;

pub enum AnimationDirection {
    Open,
    Close,
}

pub struct Animation {
    current_height: f32,
    end_value: f32,
    dir: AnimationDirection,
}

impl Animation {
    pub fn new(wrapper: &web_sys::Element, dir: AnimationDirection) -> Animation {
        let max_height = Animation::get_max_height(wrapper);
        let mut current_height = 0.0;
        let mut end_value = 0.0;

        match dir {
            AnimationDirection::Open => {
                if let Some(height) = dom::get_style_height(wrapper) {
                    current_height = height;
                }

                end_value = max_height;
            }
            AnimationDirection::Close => current_height = max_height,
        }

        Animation {
            current_height,
            dir,
            end_value,
        }
    }

    pub fn calc_step(&mut self) -> bool {
        let gap = lerp(self.current_height, self.end_value);

        match self.dir {
            AnimationDirection::Open => {
                self.current_height += gap;
                self.current_height < self.end_value - 1.0
            }
            AnimationDirection::Close => {
                self.current_height -= gap;
                self.current_height > 1.0
            }
        }
    }

    pub fn get_max_height(wrapper: &web_sys::Element) -> f32 {
        let children = wrapper.children();
        if children.length() == 1 {
            if let Some(child_node) = children.item(0) {
                child_node.client_height() as f32
            } else {
                console::error(1);
                0.0
            }
        } else {
            console::error(1);
            0.0
        }
    }
}

pub fn animation(item: &web_sys::Element, dir: AnimationDirection) {
    let ref_animation = Rc::new(RefCell::new(None));
    let animation = ref_animation.clone();

    if let Ok(wrapper) = item.query_selector(crate::SEL_WRAPPER) {
        if let Some(wrapper) = wrapper {
            let mut values = Animation::new(&wrapper, dir);

            *animation.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                if values.calc_step() {
                    dom::set_style_height(&wrapper, values.current_height);
                    dom::request_animation_frame(ref_animation.borrow().as_ref().unwrap());
                } else {
                    dom::set_style_height(&wrapper, values.end_value);
                }
            }) as Box<dyn FnMut()>));

            dom::request_animation_frame(animation.borrow().as_ref().unwrap());
        }
    }
}

fn lerp(current: f32, end: f32) -> f32 {
    let dif = current - end;
    0.3 * dif.abs()
}

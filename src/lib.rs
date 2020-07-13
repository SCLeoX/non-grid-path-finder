use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, HtmlElement};

use crate::canvas::Canvas;
use crate::input::Input;
use crate::state::State;

mod canvas;
#[macro_use]
mod console;
mod geometry;
mod input;
mod navigation;
mod state;

// pub use geometry::{MutualIntersect, Segment, Shape};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

fn body() -> HtmlElement {
    document().body().unwrap()
}

fn create_element<T: JsCast>(name: &str) -> T {
    document().create_element(name).unwrap().dyn_into::<T>().unwrap()
}

pub fn init_canvas() -> Canvas {
    let body = body();
    let canvas: HtmlCanvasElement = create_element("canvas");
    canvas.set_width(800);
    canvas.set_height(600);
    body.append_with_node_1(&canvas).unwrap();
    Canvas::new(canvas)
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
    let canvas = init_canvas();
    let state = State::new();
    let input = Input::new(canvas.html_canvas());
    start_main_loop(&state, &input, &Rc::new(RefCell::new(canvas)));
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

pub fn start_main_loop(state: &Rc<RefCell<State>>, input: &Rc<RefCell<Input>>, canvas: &Rc<RefCell<Canvas>>) {
    let callback = Rc::new(RefCell::new(None));
    let callback_ref = Rc::clone(&callback);
    let state_ref = Rc::clone(state);
    let input_ref = Rc::clone(input);
    let canvas_ctx_ref = Rc::clone(canvas);

    *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        input_ref.borrow_mut().frame_start();
        let input = input_ref.borrow();
        let mut state = state_ref.borrow_mut();
        let canvas = canvas_ctx_ref.borrow();

        state.update(&input);
        state.render(&canvas, &input);

        request_animation_frame(callback_ref.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(callback.borrow().as_ref().unwrap());
}

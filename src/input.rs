use core::mem;
use std::cell::RefCell;
use std::rc::Rc;

use crate::geometry::Vec2;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent};

#[derive(Debug)]
pub struct FramePressedKey {
    code: String,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameMouseClick {
    x: i32,
    y: i32,
}

impl FrameMouseClick {
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn pair(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Default)]
pub struct Input {
    /// Key pressed since last call to frame_start()
    collecting_frame_pressed: Option<Vec<FramePressedKey>>,
    /// Key pressed in between last two frame_start() calls
    current_frame_pressed: Option<Vec<FramePressedKey>>,
    collecting_frame_mouse_clicked: Option<FrameMouseClick>,
    current_frame_mouse_clicked: Option<FrameMouseClick>,
    mouse_x: i32,
    mouse_y: i32,
}

impl Input {
    pub fn frame_start(&mut self) {
        self.current_frame_pressed = self.collecting_frame_pressed.take();
        self.current_frame_mouse_clicked = self.collecting_frame_mouse_clicked.take();
        if let Some(mouse_click) = &self.current_frame_mouse_clicked {
            self.mouse_x = mouse_click.x;
            self.mouse_y = mouse_click.y;
        }
    }
    pub fn is_frame_key_pressed(&self, target_code: &str) -> bool {
        if let Some(frame_pressed) = &self.current_frame_pressed {
            frame_pressed.iter().any(|key| target_code == key.code)
        } else {
            false
        }
    }
    pub fn frame_mouse_clicked(&self) -> Option<FrameMouseClick> {
        self.current_frame_mouse_clicked
    }
    pub fn mouse_pos(&self) -> Vec2 {
        (self.mouse_x, self.mouse_y).into()
    }
}

fn register_key_up(input: &Rc<RefCell<Input>>) {
    let input_ref = Rc::clone(input);
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let frame_pressed = &mut input_ref.borrow_mut().collecting_frame_pressed;
        if let Some(frame_pressed_vec) = frame_pressed {
            frame_pressed_vec.push(FramePressedKey { code: event.code() });
        } else {
            *frame_pressed = Some(vec![FramePressedKey { code: event.code() }]);
        }
    }) as Box<dyn Fn(KeyboardEvent)>);
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
        .unwrap();
    // Please live forever
    mem::forget(closure);
}

fn register_mouse_click(input: &Rc<RefCell<Input>>, canvas: &HtmlCanvasElement) {
    let input_ref = Rc::clone(input);
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        input_ref
            .borrow_mut()
            .collecting_frame_mouse_clicked
            .replace(FrameMouseClick {
                x: event.offset_x(),
                y: event.offset_y(),
            });
    }) as Box<dyn Fn(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    // You too, please live forever
    mem::forget(closure);
}

fn register_mouse_move(input: &Rc<RefCell<Input>>, canvas: &HtmlCanvasElement) {
    let input_ref = Rc::clone(input);
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut input = input_ref.borrow_mut();
        input.mouse_x = event.offset_x();
        input.mouse_y = event.offset_y();
    }) as Box<dyn Fn(MouseEvent)>);
    canvas
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .unwrap();
    // Yes, we really need you guys for that long
    mem::forget(closure);
}

impl Input {
    pub fn new(canvas: &HtmlCanvasElement) -> Rc<RefCell<Input>> {
        let input = Rc::new(RefCell::new(Input::default()));
        register_key_up(&input);
        register_mouse_click(&input, &canvas);
        register_mouse_move(&input, &canvas);
        input
    }
}

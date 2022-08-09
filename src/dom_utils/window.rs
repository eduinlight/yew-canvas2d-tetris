use super::{JsAnimationFrame, Size};
use instant::Instant;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub fn get_window() -> web_sys::Window {
  web_sys::window().unwrap()
}

pub fn get_window_size(window: &web_sys::Window) -> Size {
  let width = window.inner_width().unwrap().as_f64().unwrap();
  let height = window.inner_height().unwrap().as_f64().unwrap();
  Size { width, height }
}

pub fn create_render_cicle(window: Rc<web_sys::Window>, render: Rc<dyn JsAnimationFrame>) {
  let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
  let g = f.clone();
  let previous_ts = Rc::new(RefCell::new(Instant::now()));
  *g.borrow_mut() = {
    let window = Rc::clone(&window);
    Some(Closure::<dyn FnMut()>::new(move || {
      let elapsed = (*previous_ts).borrow().clone().elapsed();
      *previous_ts.borrow_mut() = Instant::now();
      render.as_ref().frame(elapsed.as_secs_f32());
      window
        .request_animation_frame((*f).borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
    }))
  };
  window
    .request_animation_frame((*g).borrow().as_ref().unwrap().as_ref().unchecked_ref())
    .unwrap();
}

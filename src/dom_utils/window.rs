use super::{JsAnimationFrame, Size};
use instant::Instant;
use std::borrow::Borrow;
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

pub fn request_animation_frame(window: &web_sys::Window, clojure: &Closure<dyn FnMut()>) {
  window
    .request_animation_frame(clojure.as_ref().unchecked_ref())
    .unwrap();
}

pub fn create_render_cicle(window: Rc<web_sys::Window>, render: Rc<dyn JsAnimationFrame>) {
  let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
  let g = f.clone();
  let clojure = {
    let previous_ts = Rc::new(RefCell::new(Instant::now()));
    let window = Rc::clone(&window);
    let mut i = 0;
    Closure::<dyn FnMut()>::wrap(Box::new(move || {
      let elapsed = (*previous_ts).borrow().clone().elapsed().as_secs_f32();
      log::info!("elapsed {}, render {}", elapsed, i);
      *previous_ts.borrow_mut() = Instant::now();
      render.as_ref().frame(elapsed);
      request_animation_frame(window.as_ref(), (*f).borrow().as_ref().unwrap());
      i += 1;
    }))
  };
  *g.borrow_mut() = Some(clojure);
  request_animation_frame(window.as_ref(), (*g).borrow().as_ref().unwrap());
}

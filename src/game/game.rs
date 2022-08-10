use super::{entities::SmileEntity, EntityType};
use crate::dom_utils::{
  create_render_cicle, get_canvas, get_canvas_context_2d, get_window, get_window_size,
  JsAnimationFrame, Size,
};
use log::info;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};
use yew::NodeRef;

pub trait GameCicleType {
  fn handle_user_input(&self);
  fn update(&self);
  fn render(&self);
  fn start(game: Rc<RefCell<Self>>);
}

#[derive(Clone)]
pub struct Game {
  window: Rc<Window>,
  canvas: Rc<HtmlCanvasElement>,
  context: Rc<CanvasRenderingContext2d>,
  main_canvas: Rc<HtmlCanvasElement>,
  main_context: Rc<CanvasRenderingContext2d>,
  size: Size,
  entities: Vec<Rc<Box<dyn EntityType>>>,
}

impl Game {
  pub fn new(canvas_node: &NodeRef, main_canvas_node: &NodeRef) -> Self {
    let window = Rc::new(get_window());
    let canvas = Rc::new(get_canvas(&canvas_node));
    info!("{:?}", canvas);
    let context = Rc::new(get_canvas_context_2d(&Rc::clone(&canvas)));

    let main_canvas = Rc::new(get_canvas(&main_canvas_node));
    let main_context = Rc::new(get_canvas_context_2d(&main_canvas));

    let size = get_window_size(&window);

    // add entities
    let mut entities: Vec<Rc<Box<dyn EntityType>>> = vec![];
    let smile = SmileEntity::new();
    entities.push(Rc::new(Box::new(smile)));

    Game {
      window,
      canvas,
      context,
      main_canvas,
      main_context,
      size,
      entities,
    }
  }

  fn sync_window_size(&mut self) {
    self.size = get_window_size(&self.window);
    log::info!("here {:?}", self.size);
    self.canvas.set_width(self.size.width as u32);
    self.canvas.set_height(self.size.height as u32);

    self.main_canvas.set_width(self.size.width as u32);
    self.main_canvas.set_height(self.size.height as u32);
  }

  fn handle_resize(&mut self) {
    self.sync_window_size();
    {
      let mut game = self.clone();
      let clojure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::DomWindowResizeEventDetail| {
        game.sync_window_size();
      });

      self
        .window
        .add_event_listener_with_callback("resize", clojure.as_ref().unchecked_ref())
        .unwrap();
      clojure.forget();
    };
  }
}

impl JsAnimationFrame for Game {
  fn frame(&self, elapsed: f32) {
    info!("{}", elapsed);
    self.update();
    self.render();
  }
}

impl GameCicleType for Game {
  fn handle_user_input(&self) {}

  fn update(&self) {}

  fn render(&self) {
    self
      .context
      .clear_rect(0f64, 0f64, self.size.width, self.size.height);

    for entity in self.entities.iter() {
      entity.render(Rc::clone(&self.context));
    }

    // swap buffers
    self
      .main_context
      .clear_rect(0f64, 0f64, self.size.width, self.size.height);
    self
      .main_context
      .draw_image_with_html_canvas_element(&self.canvas, 0f64, 0f64)
      .unwrap();
  }

  fn start(game: Rc<RefCell<Self>>) {
    (*game).borrow_mut().handle_resize();
    let window = Rc::clone(&(*game).borrow().window);
    let game = Rc::new((*game).borrow().clone());
    create_render_cicle(window, game);
  }
}

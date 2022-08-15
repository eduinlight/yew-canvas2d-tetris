use crate::dom_utils::{
  create_render_cicle, get_canvas, get_canvas_context_2d, get_window, get_window_size,
  JsAnimationFrame, Size,
};
use crate::utils::plane::*;
use log::info;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};
use yew::NodeRef;

use super::tetris::{Board, Shape};
use super::EntityType;

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
  board: Rc<Board>,
  shapes: Vec<Rc<Shape>>,
  active_shape: Option<Rc<Shape>>,
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

    let board = Board::new(
      Point2DBuilder::<f32>::default()
        .x(50 as f32)
        .y(50 as f32)
        .build()
        .unwrap(),
      300,
      600,
    );

    let mut shapes = vec![];
    let box_shape = Rc::new(Shape::new_box(
      Point2DBuilder::<f32>::default()
        .x(board.position.x)
        .y(board.position.y)
        .build()
        .unwrap(),
      board.cell_size,
    ));

    let j_shape = Rc::new(Shape::new_j(
      Point2DBuilder::<f32>::default()
        .x(board.position.x)
        .y(board.position.y)
        .build()
        .unwrap(),
      board.cell_size,
    ));

    let l_shape = Rc::new(Shape::new_l(
      Point2DBuilder::<f32>::default()
        .x(board.position.x)
        .y(board.position.y)
        .build()
        .unwrap(),
      board.cell_size,
    ));

    let i_shape = Rc::new(Shape::new_i(
      Point2DBuilder::<f32>::default()
        .x(board.position.x)
        .y(board.position.y)
        .build()
        .unwrap(),
      board.cell_size,
    ));

    let t_shape = Rc::new(Shape::new_t(
      Point2DBuilder::<f32>::default()
        .x(board.position.x)
        .y(board.position.y)
        .build()
        .unwrap(),
      board.cell_size,
    ));

    let s_shape = Rc::new(Shape::new_s(
      Point2DBuilder::<f32>::default()
        .x(board.position.x)
        .y(board.position.y)
        .build()
        .unwrap(),
      board.cell_size,
    ));

    let sr_shape = Rc::new(Shape::new_sr(
      Point2DBuilder::<f32>::default()
        .x(board.position.x)
        .y(board.position.y)
        .build()
        .unwrap(),
      board.cell_size,
    ));

    shapes.push(Rc::clone(&box_shape));
    shapes.push(Rc::clone(&j_shape));
    shapes.push(Rc::clone(&l_shape));
    shapes.push(Rc::clone(&i_shape));
    shapes.push(Rc::clone(&t_shape));
    shapes.push(Rc::clone(&s_shape));
    shapes.push(Rc::clone(&sr_shape));

    let active_shape = Some(Rc::clone(&i_shape));

    Game {
      window,
      canvas,
      context,
      main_canvas,
      main_context,
      size,
      board: Rc::new(board),
      shapes,
      active_shape,
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

  fn swap_buffers(&self) {
    // swap buffers
    self
      .main_context
      .clear_rect(0f64, 0f64, self.size.width, self.size.height);
    self
      .main_context
      .draw_image_with_html_canvas_element(&self.canvas, 0f64, 0f64)
      .unwrap();
  }
}

impl JsAnimationFrame for Game {
  fn frame(&self, elapsed: f32) {
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

    self.board.render(Rc::clone(&self.context));

    if self.active_shape.is_some() {
      self
        .active_shape
        .clone()
        .unwrap()
        .render(Rc::clone(&self.context));
    }

    self.swap_buffers();
  }

  fn start(game: Rc<RefCell<Self>>) {
    (*game).borrow_mut().handle_resize();
    let window = Rc::clone(&(*game).borrow().window);
    let game = Rc::new((*game).borrow().clone());
    create_render_cicle(window, game);
  }
}

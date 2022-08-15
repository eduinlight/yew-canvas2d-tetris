use crate::{game::EntityType, utils::plane::Point2D};
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

use super::Cell;

pub trait Shape {
  fn position(&self) -> Point2D<f32>;
  fn center(&self) -> Point2D<f32>;
  fn cells(&self) -> Vec<Cell>;
}

impl<T> EntityType for T
where
  T: Shape,
{
  fn render(&self, context: Rc<CanvasRenderingContext2d>) {
    for i in self.cells().iter() {
      i.render(Rc::clone(&context));
    }
  }
}

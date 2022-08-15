use super::{Cell, CellBuilder, Shape};
use crate::{
  game::EntityType,
  utils::plane::{Point2D, Point2DBuilder},
};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LShape {
  position: Point2D<f32>,
  center: Point2D<f32>,
  cells: Vec<Cell>,
}

impl LShape {
  pub fn new(position: Point2D<f32>, cell_size: i32) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let cell_base = CellBuilder::default()
      .bg_color(String::from("#FF971C"))
      .border_color(String::from("#1d1d1d"))
      .size(cell_size)
      .position(position.clone())
      .empty(false)
      .build()
      .unwrap();
    let center = Point2DBuilder::<f32>::default()
      .x(position.x as f32 + cell_size as f32 * 1.5)
      .y(position.y as f32 + cell_size as f32 * 1.5)
      .build()
      .unwrap();

    for i in 0..2 {
      for j in 0..3 {
        if i == 0 && j < 2 {
          continue;
        }
        let mut cell = cell_base.clone();
        cell.position = Point2DBuilder::<f32>::default()
          .x(cell_base.position.x + cell_size as f32 * j as f32)
          .y(cell_base.position.y + cell_size as f32 * i as f32)
          .build()
          .unwrap();
        cells.push(cell);
      }
    }

    LShape {
      position,
      cells,
      center,
    }
  }
}

impl Shape for LShape {
  fn position(&self) -> Point2D<f32> {
    self.position
  }

  fn center(&self) -> Point2D<f32> {
    self.center
  }

  fn cells(&self) -> Vec<Cell> {
    self.cells
  }
}
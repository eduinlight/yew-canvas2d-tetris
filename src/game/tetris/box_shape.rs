use super::{Cell, CellBuilder, Shape};
use crate::utils::plane::{Point2D, Point2DBuilder};

#[derive(Debug, Clone)]
pub struct BoxShape {
  position: Point2D<f32>,
  cells: Vec<Cell>,
  center: Point2D<f32>,
}

impl BoxShape {
  pub fn new(position: Point2D<f32>, cell_size: i32) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let cell_base = CellBuilder::default()
      .bg_color(String::from("#FFD500"))
      .border_color(String::from("#1d1d1d"))
      .size(cell_size)
      .position(position.clone())
      .empty(false)
      .build()
      .unwrap();

    let center = Point2DBuilder::<f32>::default()
      .x(position.x as f32 + cell_size as f32)
      .y(position.y as f32 + cell_size as f32)
      .build()
      .unwrap();

    for i in 0..2 {
      for j in 0..2 {
        let mut cell = cell_base.clone();
        cell.position = Point2DBuilder::<f32>::default()
          .x(cell_base.position.x + cell_size as f32 * j as f32)
          .y(cell_base.position.y + cell_size as f32 * i as f32)
          .build()
          .unwrap();
        cells.push(cell);
      }
    }

    BoxShape {
      position,
      cells,
      center,
    }
  }
}

impl Shape for BoxShape {
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

use super::{Cell, CellBuilder};
use crate::{
  game::EntityType,
  utils::plane::{Point2D, Point2DBuilder},
};
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct Shape {
  position: Point2D<f32>,
  cells: Vec<Cell>,
  center: Point2D<f32>,
  cell_size: i32,
}

impl Shape {
  pub fn new_box(position: Point2D<f32>, cell_size: i32) -> Self {
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

    Shape {
      position,
      cells,
      center,
      cell_size,
    }
  }

  pub fn new_j(position: Point2D<f32>, cell_size: i32) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let cell_base = CellBuilder::default()
      .bg_color(String::from("#0341AE"))
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
        if i == 0 && j > 0 {
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

    Shape {
      position,
      cells,
      center,
      cell_size,
    }
  }

  pub fn new_l(position: Point2D<f32>, cell_size: i32) -> Self {
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

    Shape {
      position,
      cells,
      center,
      cell_size,
    }
  }

  pub fn new_t(position: Point2D<f32>, cell_size: i32) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let cell_base = CellBuilder::default()
      .bg_color(String::from("#FF3213"))
      .border_color(String::from("#1d1d1d"))
      .size(cell_size)
      .position(position.clone())
      .empty(false)
      .build()
      .unwrap();

    let center = Point2DBuilder::<f32>::default()
      .x(position.x as f32 + cell_size as f32 * 1.5)
      .y(position.y as f32 + cell_size as f32 * 0.5)
      .build()
      .unwrap();

    for i in 0..2 {
      for j in 0..3 {
        if i == 1 && (j == 0 || j == 2) {
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

    Shape {
      position,
      cells,
      center,
      cell_size,
    }
  }

  pub fn new_s(position: Point2D<f32>, cell_size: i32) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let cell_base = CellBuilder::default()
      .bg_color(String::from("#808000"))
      .border_color(String::from("#1d1d1d"))
      .size(cell_size)
      .position(position.clone())
      .empty(false)
      .build()
      .unwrap();

    let center = Point2DBuilder::<f32>::default()
      .x(position.x as f32 + cell_size as f32 * 1.5)
      .y(position.y as f32 + cell_size as f32 * 0.5)
      .build()
      .unwrap();

    for i in 0..2 {
      for j in 0..3 {
        if (i == 0 && j == 0) || (i == 1 && j == 2) {
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

    Shape {
      position,
      cells,
      center,
      cell_size,
    }
  }

  pub fn new_sr(position: Point2D<f32>, cell_size: i32) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let cell_base = CellBuilder::default()
      .bg_color(String::from("#32CD32"))
      .border_color(String::from("#1d1d1d"))
      .size(cell_size)
      .position(position.clone())
      .empty(false)
      .build()
      .unwrap();

    let center = Point2DBuilder::<f32>::default()
      .x(position.x as f32 + cell_size as f32 * 1.5)
      .y(position.y as f32 + cell_size as f32 * 0.5)
      .build()
      .unwrap();

    for i in 0..2 {
      for j in 0..3 {
        if (i == 0 && j == 2) || (i == 1 && j == 0) {
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

    Shape {
      position,
      cells,
      center,
      cell_size,
    }
  }

  pub fn new_i(position: Point2D<f32>, cell_size: i32) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let cell_base = CellBuilder::default()
      .bg_color(String::from("#00c0cf"))
      .border_color(String::from("#1d1d1d"))
      .size(cell_size)
      .position(position.clone())
      .empty(false)
      .build()
      .unwrap();

    let center = Point2DBuilder::<f32>::default()
      .x(position.x as f32 + cell_size as f32 * 2.0)
      .y(position.y as f32 + cell_size as f32)
      .build()
      .unwrap();

    for i in 0..4 {
      let mut cell = cell_base.clone();
      cell.position = Point2DBuilder::<f32>::default()
        .x(cell_base.position.x + cell_size as f32 * i as f32)
        .y(cell_base.position.y)
        .build()
        .unwrap();
      cells.push(cell);
    }

    Shape {
      position,
      cells,
      center,
      cell_size,
    }
  }

  pub fn move_right(&mut self) {}

  pub fn move_left(&mut self) {}

  pub fn rotate_right(&mut self) {}

  pub fn rotate_left(&mut self) {}
}

impl EntityType for Shape {
  fn render(&self, context: Rc<CanvasRenderingContext2d>) {
    for i in self.cells.iter() {
      i.render(Rc::clone(&context));
    }
  }
}

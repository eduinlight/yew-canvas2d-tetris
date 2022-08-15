use crate::{
  game::EntityType,
  utils::plane::{Point2D, Point2DBuilder},
};
use std::rc::Rc;

use super::{Cell, CellBuilder};

const BOARD_ROWS: i32 = 24;
const BOARD_COLS: i32 = 10;
const START_VISIBLE_ROW: usize = 4;
const VISIBLE_ROWS: usize = 20;

#[derive(Clone, Debug)]
pub struct Board {
  pub position: Point2D<f32>,
  pub width: i32,
  pub height: i32,
  pub cells: Vec<Vec<Cell>>,
  pub cell_size: i32,
  pub board_rows: i32,
  pub board_cols: i32,
  pub start_visible_row: usize,
  pub visible_rows: usize,
}

impl Board {
  pub fn new(position: Point2D<f32>, w: i32, h: i32) -> Self {
    let mut cells: Vec<Vec<Cell>> = vec![];
    let cell_size = w / BOARD_COLS;

    for i in 0..BOARD_ROWS {
      let mut row: Vec<Cell> = vec![];
      for j in 0..BOARD_COLS {
        let position = Point2DBuilder::<f32>::default()
          .x(position.x + j as f32 * cell_size as f32)
          .y(position.y + i as f32 * cell_size as f32)
          .build()
          .unwrap();
        row.push(
          CellBuilder::default()
            .position(position)
            .bg_color(String::from("#161614"))
            .border_color(String::from("#040703"))
            .size(cell_size)
            .empty(true)
            .build()
            .unwrap(),
        );
      }
      cells.push(row);
    }

    let board = Board {
      position,
      width: w,
      height: h,
      cells,
      cell_size,
      board_rows: BOARD_ROWS,
      board_cols: BOARD_COLS,
      start_visible_row: START_VISIBLE_ROW,
      visible_rows: VISIBLE_ROWS,
    };

    board
  }
}

impl EntityType for Board {
  fn render(&self, context: std::rc::Rc<web_sys::CanvasRenderingContext2d>) {
    log::info!("{}", self.cells.len());
    let wait_rows = &self.cells[0..START_VISIBLE_ROW];
    for row in wait_rows.iter() {
      for cell in row.iter() {
        if !cell.empty {
          cell.render(Rc::clone(&context));
        }
      }
    }

    let visible_rows = &self.cells[START_VISIBLE_ROW..START_VISIBLE_ROW + VISIBLE_ROWS];
    for row in visible_rows.iter() {
      for cell in row.iter() {
        cell.render(Rc::clone(&context));
      }
    }
  }
}

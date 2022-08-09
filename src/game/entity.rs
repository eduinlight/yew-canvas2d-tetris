use std::{fmt::Debug, rc::Rc};

use web_sys::CanvasRenderingContext2d;

pub trait EntityType {
  fn render(&self, context: Rc<CanvasRenderingContext2d>);
}

impl Debug for dyn EntityType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", format!("EntityType"))?;
    Ok(())
  }
}

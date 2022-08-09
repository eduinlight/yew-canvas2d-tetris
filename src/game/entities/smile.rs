use crate::game::EntityType;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct SmileEntity {}

impl SmileEntity {
  pub fn new() -> Self {
    SmileEntity {}
  }
}

impl EntityType for SmileEntity {
  fn render(&self, context: std::rc::Rc<web_sys::CanvasRenderingContext2d>) {
    context.begin_path();

    // Draw the outer circle.
    context.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context.arc(60.0, 65.0, 5.0, 0.0, PI * 2.0).unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context.arc(90.0, 65.0, 5.0, 0.0, PI * 2.0).unwrap();

    context.stroke();
  }
}

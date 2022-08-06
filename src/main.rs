use log::info;
use std::rc::Rc;
use std::{borrow::Borrow, f64};
use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

mod dom_utils;
use dom_utils::*;

#[function_component]
fn App() -> Html {
  let main_canvas_node = use_node_ref();
  let buffer_canvas_node = use_node_ref();
  {
    let main_canvas_node = main_canvas_node.clone();
    let buffer_canvas_node = buffer_canvas_node.clone();
    use_effect(move || {
      let window = Rc::new(get_window());
      let canvas = Rc::new(get_canvas(&buffer_canvas_node));
      let context = Rc::new(get_canvas_context_2d(&Rc::clone(&canvas)));

      let main_canvas = Rc::new(get_canvas(&main_canvas_node));
      let main_context = Rc::new(get_canvas_context_2d(&main_canvas));

      {
        let canvas = Rc::clone(&canvas);
        let window = Rc::clone(&window);
        let size = get_window_size(&window);
        canvas.set_width(size.width as u32);
        canvas.set_height(size.height as u32);

        main_canvas.set_width(size.width as u32);
        main_canvas.set_height(size.height as u32);
      };

      {
        let clojure = {
          let canvas = Rc::clone(&canvas);
          let main_canvas = Rc::clone(&main_canvas);
          let window = Rc::clone(&window);
          Closure::<dyn FnMut(_)>::new(move |_: web_sys::DomWindowResizeEventDetail| {
            let size = get_window_size(&window);
            canvas.set_width(size.width as u32);
            canvas.set_height(size.height as u32);
            main_canvas.set_width(size.width as u32);
            main_canvas.set_height(size.height as u32);
          })
        };

        window
          .add_event_listener_with_callback("resize", clojure.as_ref().unchecked_ref())
          .unwrap();
        clojure.forget();
      };

      let draw = {
        let context = Rc::clone(&context);
        let window = Rc::clone(&window);
        move || {
          let size = get_window_size(&window);
          context.clear_rect(0f64, 0f64, size.width, size.height);
          context.begin_path();

          // Draw the outer circle.
          context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

          // Draw the mouth.
          context.move_to(110.0, 75.0);
          context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

          // Draw the left eye.
          context.move_to(65.0, 65.0);
          context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

          // Draw the right eye.
          context.move_to(95.0, 65.0);
          context
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

          context.stroke();

          // swap buffers
          main_context.clear_rect(0f64, 0f64, size.width, size.height);
          main_context
            .draw_image_with_html_canvas_element(canvas.borrow(), 0f64, 0f64)
            .unwrap();
        }
      };

      let update = {
        let context = Rc::clone(&context);
        move || {}
      };

      {
        let draw = draw.clone();
        let update = update.clone();
        let render = move |elapsed: f32| {
          info!("{}", elapsed);
          update();
          draw();
        };
        create_render_cicle(Box::new(window), Box::new(render));
      };

      || {}
    });
  }

  html! {
    <>
      <canvas ref={main_canvas_node} width={100} height={100} />
      <canvas ref={buffer_canvas_node} width={100} height={100} class="hidden" />
    </>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}

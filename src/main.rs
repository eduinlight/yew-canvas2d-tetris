use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

mod dom_utils;
mod game;
mod utils;

use game::*;

#[function_component]
fn App() -> Html {
  let main_canvas_node = use_node_ref();
  let buffer_canvas_node = use_node_ref();
  let game = use_state(|| None);

  {
    let game = game.clone();
    let main_canvas_node = main_canvas_node.clone();
    let buffer_canvas_node = buffer_canvas_node.clone();
    use_effect(move || {
      if game.is_none() {
        log::info!("running");
        let tmp = Rc::new(RefCell::new(Game::new(
          &main_canvas_node,
          &buffer_canvas_node,
        )));
        game.set(Some(Rc::clone(&tmp)));
        Game::start(Rc::clone(&tmp));
      }
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

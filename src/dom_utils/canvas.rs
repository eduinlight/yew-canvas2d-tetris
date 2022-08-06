use wasm_bindgen::JsCast;
use yew::NodeRef;

pub fn get_canvas(canvas_node: &NodeRef) -> web_sys::HtmlCanvasElement {
  canvas_node.cast::<web_sys::HtmlCanvasElement>().unwrap()
}

pub fn get_canvas_context_2d(
  canvas: &web_sys::HtmlCanvasElement,
) -> web_sys::CanvasRenderingContext2d {
  canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap()
}

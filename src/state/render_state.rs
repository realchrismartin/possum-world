use crate::wasm_bindgen;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use std::option::Option;

#[wasm_bindgen]
pub struct RenderState
{
    context: Option<WebGl2RenderingContext>
}

#[wasm_bindgen]
impl RenderState
{
    pub fn new() -> Self
    {
        Self
        {
            context: None,
        }
    }

    pub fn set_context(&mut self, document : Document)
    {
        let canvas = document.get_element_by_id("canvas").unwrap();

        let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Failed to cast canvas to HtmlCanvasElement");

        let context = canvas
            .get_context("webgl2")
            .expect("Failed to find WebGL Context")
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .expect("Failed to cast WebGL Context");

        self.context = Some(context);
    }
}
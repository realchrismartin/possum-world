use crate::wasm_bindgen;
use web_sys::Document;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;
use crate::graphics::shader::Shader;
use crate::util::logging::log;
use std::option::Option;

#[wasm_bindgen]
pub struct RenderState
{
    context: WebGl2RenderingContext,
    shader: Option<Shader>
}

#[wasm_bindgen]
impl RenderState
{
    pub fn new(document : &Document) -> Result<RenderState,JsValue> 
    {
        let canvas = match document.get_element_by_id("canvas")
        {
            Some(canvas) => canvas,
            None => return Err(JsValue::from_str("Failed to find canvas element"))
        };
        
        let canvas = match canvas.dyn_into::<web_sys::HtmlCanvasElement>()
        {
            Ok(canvas) => canvas,
            Err(e) => return Err(JsValue::from_str("Failed to cast canvas to HtmlCanvasElement"))
        };

        let context = match canvas.get_context("webgl2")
        {
            Ok(context) =>context,
            Err(e) => return Err(JsValue::from_str("Failed to find WebGL Context"))
        };

        let web_context = match context.unwrap().dyn_into::<WebGl2RenderingContext>()
        {
            Ok(context) =>context,
            Err(e) => return Err(JsValue::from_str("Failed to load WebGl2RenderingContext from context"))
        };

        Ok(Self
        {
            context: web_context,
            shader: None::<Shader>
        })
    }

    pub fn set_shader(&mut self, vertex_source :&str, frag_source: &str)
    {
        let shader = match Shader::new(&self.context,vertex_source,frag_source)
        {
            Ok(shader) => shader,
            Err(e) => {
                log(e.as_str());
                return;
            }
        };

        log("Loaded shader!");

        self.shader = Some(shader);
    }
}
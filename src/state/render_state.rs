use crate::wasm_bindgen;

#[wasm_bindgen]
pub struct RenderState
{

}

#[wasm_bindgen]
impl RenderState
{
    pub fn new() -> Self
    {

        Self
        {

        }
    }

    pub fn set_context(self : &mut Self)
    {
            /*
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas= canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;
        */


    }
}
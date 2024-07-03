use web_sys::WebGl2RenderingContext;
use web_sys::HtmlImageElement;
use wasm_bindgen::JsValue;
use std::cmp::max;
pub struct Texture
{
    level: i32,
    internal_format: u32,
    src_format: u32,
    src_type: u32,
    width: u32,
    height: u32
}

impl Texture
{
    pub fn new() -> Self 
    {
        Self{
            level: 0,
            internal_format: WebGl2RenderingContext::RGBA,
            src_format: WebGl2RenderingContext::RGBA,
            src_type: WebGl2RenderingContext::UNSIGNED_BYTE,
            width: 1,
            height:1 
        }
    }

    pub fn load(&mut self, context: &WebGl2RenderingContext, img: HtmlImageElement, texture_number: u32) -> Result<(),JsValue>
    {
        let texture = context.create_texture().expect("Cannot create texture");
        context.active_texture(texture_number);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        
        context.tex_image_2d_with_u32_and_u32_and_html_image_element(
        WebGl2RenderingContext::TEXTURE_2D,
        self.level,
        self.internal_format as i32,
        self.src_format,
        self.src_type,
        &img
        ).expect("Error binding.");

        self.height = max(img.height(),1);
        self.width = max(img.width(),1);

        Ok(())
    }

    pub fn get_dimensions(&self) -> [i32;2]
    {
        return [self.width as i32,self.height as i32];
    }

}
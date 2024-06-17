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

    pub fn get_sprite_coordinates(&self,top_left_pixel_coordinate: [i32;2], dimensions: [i32;2]) -> [[f32;2];4]
    {
        let width_ratio = 1.0 / self.width as f32;
        let height_ratio = 1.0 / self.height as f32;

        let left_bottom = [width_ratio * top_left_pixel_coordinate[0] as f32,(height_ratio * top_left_pixel_coordinate[1] as f32) + dimensions[1] as f32 * height_ratio ];
        let left_top = [left_bottom[0],left_bottom[1] - dimensions[1] as f32 * height_ratio];
        let right_bottom = [left_bottom[0] + dimensions[0] as f32 * width_ratio,left_bottom[1]];
        let right_top = [right_bottom[0],left_top[1]];

        return [left_top,left_bottom,right_bottom,right_top];
    }

    pub fn load(&mut self, context: &WebGl2RenderingContext, img: HtmlImageElement) -> Result<(),JsValue>
    {
        let texture = context.create_texture().expect("Cannot create texture");
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
        
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
}
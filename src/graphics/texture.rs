use web_sys::WebGl2RenderingContext;
use wasm_bindgen::JsValue;

pub struct Texture
{
    level: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    border: i32,
    src_format: u32,
    src_type: u32
}

impl Texture
{
    pub fn new(image_source: &str) -> Self 
    {
        Self{
            level: 0,
            internal_format: WebGl2RenderingContext::RGBA,
            width: 1,
            height: 1,
            border: 0,
            src_format: WebGl2RenderingContext::RGBA,
            src_type: WebGl2RenderingContext::UNSIGNED_BYTE
        }
    }

    pub fn load(&self, context: &WebGl2RenderingContext) -> Result<(),JsValue>
    {
        let pixel: [u8; 4] = [0, 0, 255, 255];

        let texture = context.create_texture().expect("Cannot create gl texture");
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            self.level,
            self.internal_format as i32,
            self.width,
            self.height,
            self.border,
            self.src_format,
            self.src_type,
            Some(&pixel),
        )?;


        Ok(())
    }
}
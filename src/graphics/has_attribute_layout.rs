use web_sys::WebGlVertexArrayObject;

pub trait HasAttributeLayout
{
    fn generate_vao() -> Result<WebGlVertexArrayObject,String>;
}
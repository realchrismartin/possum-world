use web_sys::{WebGl2RenderingContext,WebGlVertexArrayObject};
use std::option::Option;

pub trait HasAttributeLayout
{
    fn generate_vao(context: &WebGl2RenderingContext) -> Option<WebGlVertexArrayObject>;
}
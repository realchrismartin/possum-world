use web_sys::WebGl2RenderingContext;
use std::option::Option;

pub trait Renderable
{
    fn init_vertex_layout(context: &WebGl2RenderingContext);
    fn get_vertices(&self) -> &[f32];
    fn get_indices(&self) -> &[u32];
}
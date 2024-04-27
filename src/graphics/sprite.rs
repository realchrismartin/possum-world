use web_sys::{WebGl2RenderingContext,WebGlVertexArrayObject,WebGlBuffer};
use std::option::Option;
use crate::graphics::renderable::Renderable;

pub struct Sprite
{
    vertices: [f32;12],
    indices: [u32;6]
}

impl Sprite
{

    pub fn new(vertices: [f32;12], indices: [u32;6]) -> Self 
    {

        Sprite 
        {
            vertices: vertices,
            indices: indices
        }
    }
}


impl Renderable for Sprite
{
    fn init_vertex_layout(context: &WebGl2RenderingContext)
    {
        let position_attribute_location = 0; //Hardcoded. Needs to match wahtever shaders are used for this.

        //Enable each vertex attribute
        //When this happens, whatever bound VBO there is becomes associated with this VAO.
        //We assume the VAO/VBO is already bound
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        context.enable_vertex_attrib_array(position_attribute_location as u32);
    }

    fn get_vertices(&self) -> &[f32]
    {
        return &self.vertices;
    }

    fn get_indices(&self) -> &[u32]
    {
        return &self.indices;
    }
}
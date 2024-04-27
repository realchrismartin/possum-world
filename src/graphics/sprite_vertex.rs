use web_sys::{WebGl2RenderingContext,WebGlVertexArrayObject};
use std::option::Option;
use crate::graphics::has_attribute_layout::HasAttributeLayout;

pub struct SpriteVertex
{

}

impl SpriteVertex
{
    pub fn new() -> SpriteVertex
    {
        SpriteVertex
        {

        }
    }
}

impl HasAttributeLayout for SpriteVertex
{
    fn generate_vao(context: &WebGl2RenderingContext) -> Option<WebGlVertexArrayObject>
    {
        let position_attribute_location = 0; //Hardcoded. Needs to match wahtever shaders are ussed for this.

        let vao = match context.create_vertex_array()
        {
            Some(vao) => vao,
            None => return None
        };

        context.bind_vertex_array(Some(&vao));

        //Enable each vertex attribute
        //When this happens, whatever bound VBO there is becomes associated with this VAO.
        //We assume the VBO is already bound
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.bind_vertex_array(Some(&vao)); //This is wrong TODOO
        //context.bind_vertex_array(None);

        Some(vao)
    }
}
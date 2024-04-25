use crate::util::logging::log;
use web_sys::WebGlVertexArrayObject;
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
    fn generate_vao() -> Result<WebGlVertexArrayObject,String>
    {
        //TODO: generate a Sprite VAO
        log("We are generating a vao");

        /*
        let position_attribute_location = context.get_attrib_location(&program, "position");
        let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;

        context.bind_vertex_array(Some(&vao));

        //TODO: bind the VBO's GL_ARRAY_BUFFER (glBindBuffer)
        
        //TODO: bind the VBO's GL_ELEMENT_ARRAY_BUFFER

        //Enable each vertex attribute
        //When this happens, whatever bound VBO there is becomes associated with this VAO.
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        //TODO: unbind the VAO
        */

        Err("".to_string())
    }
}
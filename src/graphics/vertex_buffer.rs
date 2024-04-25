use web_sys::{WebGlVertexArrayObject,WebGlBuffer};
use crate::graphics::has_attribute_layout::HasAttributeLayout;
use std::marker::PhantomData;

pub struct VertexBuffer<T>
{
    _phantom: PhantomData<T>, //Exists so that we can imply that <T> is associated with the VAO.
    vao: WebGlVertexArrayObject,
    /*
    vbo: WebGLBuffer,
    ebo: WebGLBuffer
    */
}

impl<T: HasAttributeLayout> VertexBuffer<T>
{
    pub fn new() -> Result<Self,String> 
    {
        //Bind the VBO before the VAO so that the VAO attributes point to it.
        /*
        let vbo = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
        */

        let vao = match T::generate_vao()
        {
            Ok(vao) => vao,
            Err(e) => return Err(e)
        };

        //Rebind the VAO
        //TODO

        //Bind the EBO after the VAO exists, so that it is associated (its state is not global)
        /*
        let ebo = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        */

        //Unbind everything

        Ok(VertexBuffer
        {
            _phantom: PhantomData,
            vao: vao
        })
    }

    pub fn bind()
    {
        //TODO: bind the VAO, presumably the VAO is already set up and associated with this VB's VBO and EBO. 
    }
}

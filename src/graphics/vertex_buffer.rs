use web_sys::{WebGl2RenderingContext,WebGlVertexArrayObject,WebGlBuffer};
use crate::graphics::has_attribute_layout::HasAttributeLayout;
use std::marker::PhantomData;
use std::option::Option;
use web_sys::js_sys::Float32Array;
use web_sys::js_sys::Uint32Array;

pub struct VertexBuffer<T>
{
    _phantom: PhantomData<T>, //Exists so that we can imply that <T> is associated with the VAO.
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    ebo: WebGlBuffer,
    index_count: usize 
}

impl<T: HasAttributeLayout> VertexBuffer<T>
{
    pub fn new(context : &WebGl2RenderingContext) -> Option<Self>
    {
        let vbo = match context.create_buffer()
        {
            Some(vbo) => vbo,
            None => return None
        };

        let ebo = match context.create_buffer()
        {
            Some(ebo) => ebo,
            None => return None
        };

        //Bind the VBO so that generated VAO will use it.
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        let vao = match T::generate_vao(context)
        {
            Some(vao) => vao,
            None => return None
        };

        context.bind_vertex_array(Some(&vao)); //Ensure VAO is still bound for EBO assoc
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,Some(&ebo)); //Assoc ebo with the vao

        //Unbind everything to ensure that the context is clean now.
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        context.bind_vertex_array(None); 

        Some(VertexBuffer
        {
            _phantom: PhantomData,
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            index_count: 0 
        })
    }

    pub fn bind(&self, context: &WebGl2RenderingContext)
    {
        context.bind_vertex_array(Some(&self.vao)); //Should bind buffers
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vbo)); //TODO
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.ebo)); //TODO
    }

    pub fn unbind(context: &WebGl2RenderingContext)
    {
        context.bind_vertex_array(None); 
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }

    pub fn buffer_data(&mut self, context: &WebGl2RenderingContext, vertex_data: &[f32], index_data: &[u32])
    {
        //TODO: this constantly recreates the buffers UGH
        //TODO: assumes we are bound
        unsafe {

            let vertices_view = Float32Array::view(vertex_data);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            let indices_view = Uint32Array::view(index_data);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        self.index_count = index_data.len();
    }
}

use crate::util::logging::log;
use web_sys::{WebGl2RenderingContext,WebGlVertexArrayObject,WebGlBuffer};
use crate::graphics::renderable::Renderable;
use std::convert::TryInto;
use std::marker::PhantomData;
use std::option::Option;
use web_sys::js_sys::Float32Array;
use web_sys::js_sys::Uint32Array;
use std::mem;

static MAX_VERTICES : usize = 100;
static MAX_INDICES : usize = 100;

pub struct VertexBuffer<T>
{
    _phantom: PhantomData<T>, //Exists so that we can imply that <T> is associated with the VAO.
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    ebo: WebGlBuffer,
    current_vertex_count: usize,
    current_index_count: usize,
}

impl<T: Renderable> VertexBuffer<T>
{
    pub fn new(context : &WebGl2RenderingContext) -> Option<Self>
    {
        let vao = match context.create_vertex_array()
        {
            Some(vao) => vao,
            None => return None
        };

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

        context.bind_vertex_array(Some(&vao));
        
        //Bind buffers so that they are associated with the VAO.
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
        context.buffer_data_with_i32(WebGl2RenderingContext::ARRAY_BUFFER, (MAX_VERTICES * std::mem::size_of::<f32>()) as i32, WebGl2RenderingContext::STATIC_DRAW);

        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,Some(&ebo)); 
        context.buffer_data_with_i32(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, (MAX_INDICES * std::mem::size_of::<u32>()) as i32, WebGl2RenderingContext::STATIC_DRAW);

        //Associate the VBO and set up vertex attributes
        //init_vertex_attributes assumes the VAO is bound
        T::init_vertex_layout(&context);

        //Unbind everything to ensure that the context is clean now.
        Self::unbind(&context);

        Some(VertexBuffer
        {
            _phantom: PhantomData,
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            current_vertex_count: 0,
            current_index_count: 0
        })
    }

    pub fn bind(&self, context: &WebGl2RenderingContext)
    {
        context.bind_vertex_array(Some(&self.vao)); //Should bind buffers
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vbo)); //TODO: should not need to bind this. Not sure why VBO is not associated with VAO.
    }

    pub fn unbind(context: &WebGl2RenderingContext)
    {
        context.bind_vertex_array(None); 
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }

    pub fn get_index_count(&self) -> usize
    {
        return self.current_index_count;
    }

    pub fn clear_data(&mut self)
    {
        //TODO: overwrite data, or don't.
        self.current_index_count = 0;
        self.current_vertex_count = 0;
    }

    pub fn buffer_data(&mut self, context: &WebGl2RenderingContext, renderable: &T)
    {
        //TODO: assumes we are bound

        let vertex_data = renderable.get_vertices();
        let index_data = renderable.get_indices();

        let new_total_vertices = vertex_data.len() + self.current_vertex_count;
        let new_total_indices = index_data.len() + self.current_index_count;

        if new_total_vertices > MAX_VERTICES
        {
            return;
        }

        if new_total_indices > MAX_INDICES
        {
            return;
        }

        let vertex_offset = (mem::size_of::<f32>() * self.current_vertex_count) as i32;
        let index_offset = (mem::size_of::<u32>() * self.current_index_count) as i32;

        //Create a new index data array to offset
        //TODO: magic 3 because of vertex data
        let new_indices : Vec<u32> = index_data.iter().map(|&x| x + (self.current_vertex_count as u32 / 3)).collect();

        log(format!("{:?}", new_indices).as_str());

        unsafe 
        {
            let vertices_view = Float32Array::view(vertex_data);
            let indices_view = Uint32Array::view(new_indices.as_slice());
            
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                vertex_offset,
                &vertices_view
                );
            
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                index_offset,
                &indices_view
            );
        }

        self.current_vertex_count = new_total_vertices;
        self.current_index_count = new_total_indices;
    }
}

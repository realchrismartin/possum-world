use web_sys::{WebGl2RenderingContext,WebGlVertexArrayObject,WebGlBuffer};
use crate::graphics::renderable::Renderable;
use std::marker::PhantomData;
use std::option::Option;
use web_sys::js_sys::Float32Array;
use web_sys::js_sys::Uint32Array;
use std::mem;
use std::ops::Range;
use std::collections::HashMap;
use crate::util::logging::log;

static MAX_VERTICES : usize = 100000; //TODO: confirm
static MAX_INDICES : usize = 100000;

pub struct BufferRanges
{
    vertex_range: Range<i32>,
    index_range: Range<i32>
}

impl BufferRanges
{
    pub fn new(vertex_range: Range<i32>, index_range: Range<i32>) -> Self
    {
        Self
        {
            vertex_range,
            index_range
        }
    }

    pub fn get_index_range(&self) -> &Range<i32>
    {
        &&self.index_range
    }

    pub fn get_vertex_range(&self) -> &Range<i32>
    {
        &&self.vertex_range
    }

    fn get_index_count(&self) -> i32 
    {
        &self.index_range.end - &self.index_range.start
    }

    fn get_vertex_count(&self) -> i32
    {
        &self.vertex_range.end - &self.vertex_range.start
    }

    fn could_contain(&self, vertex_count: usize, index_count: usize) -> bool
    {
        if self.get_vertex_count() < vertex_count as i32
        {
            return false;
        }

        if self.get_index_count() < index_count as i32
        {
            return false;
        }

        true
    }

    pub fn try_split(&mut self, vertex_count: usize, index_count: usize) -> Option<BufferRanges>
    {
        if !self.could_contain(vertex_count,index_count)
        {
            return None;
        }

        //Return the first contiguous part of the range that can fit the new data
        //Update ourselves to be the second part of the range

        log(&format!("Split a buffer of {} vertices and {} indices for a drawable, starting at {} (v) {} (i). Original buffer size: {}(v) {}(i)",vertex_count,index_count,self.get_vertex_offset(),self.get_index_offset(),self.get_vertex_count(),self.get_index_count()));

        //The returned buffer is the first piece
        let split_buffer = BufferRanges::new(
            Range::<i32>{ start: self.get_vertex_offset(), end: self.get_vertex_offset() + vertex_count as i32},
            Range::<i32>{ start: self.get_index_offset(), end: self.get_index_offset() + index_count as i32},
        );

        //This buffer's first piece is cut off
        //TODO: off by 1?
        self.set_vertex_range_start(self.get_vertex_offset() + vertex_count as i32);
        self.set_index_range_start(self.get_index_offset() + index_count as i32);
        Some(split_buffer)
    }
    
    pub fn get_index_offset(&self) -> i32
    {
        self.index_range.start
    } 

    pub fn get_vertex_offset(&self) -> i32 
    {
        self.vertex_range.start 
    }

    fn set_vertex_range_start(&mut self, start: i32)
    {
        self.vertex_range.start = start;
    }

    fn set_index_range_start(&mut self, start: i32)
    {
        self.index_range.start = start;
    }

}

pub struct VertexBuffer<T>
{
    _phantom: PhantomData<T>, //Exists so that we can imply that <T> is associated with the VAO.
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    uid_to_buffer_ranges: HashMap<u32,BufferRanges>,
    free_buffer_ranges: Vec<BufferRanges>
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

        //NB: we don't hold onto the EBO here since we don't need to read it
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,Some(&ebo)); 
        context.buffer_data_with_i32(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, (MAX_INDICES * std::mem::size_of::<u32>()) as i32, WebGl2RenderingContext::STATIC_DRAW);

        //Associate the VBO and set up vertex attributes
        //init_vertex_layout assumes the VAO we created above is bound
        T::init_vertex_layout(&context);

        //Unbind everything to ensure that the context is clean now.
        Self::unbind(&context);

        //Start out with the entire buffer free
        let buffer_range = BufferRanges::new(
            Range::<i32>{ start: 0, end: MAX_VERTICES as i32},
            Range::<i32>{ start: 0, end: MAX_INDICES as i32},
        );

        Some(VertexBuffer
        {
            _phantom: PhantomData,
            vao: vao,
            vbo: vbo,
            uid_to_buffer_ranges: HashMap::new(),
            free_buffer_ranges: vec![buffer_range]
        })
    }

    pub fn bind(&self, context: &WebGl2RenderingContext)
    {
        context.bind_vertex_array(Some(&self.vao)); //Should bind buffers

        //TODO: should not need to bind this. Not sure why VBO is not associated with VAO.
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vbo)); 
    }

    pub fn unbind(context: &WebGl2RenderingContext)
    {
        context.bind_vertex_array(None); 
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }

    pub fn clear(&mut self)
    {
        //NB: the data on the buffer still lives, but we will write over it before we use it.
        self.uid_to_buffer_ranges.clear();
    }

    pub fn get_draw_range_for_uid(&self, uid: &u32) -> Option<&Range<i32>>
    {
        if !self.uid_to_buffer_ranges.contains_key(uid)
        {
            return None;
        }

        let range = match self.uid_to_buffer_ranges.get(uid)
        {
            Some(r) => r.get_index_range(),
            None => {return None;}
        };

        Some(range)
    }

    pub fn free(&mut self, uid: &u32)
    {
        let occupied_range = match self.uid_to_buffer_ranges.remove(uid)
        {
            Some(r) => r,
            None => {
                log(&format!("Tried to free a range that doesnt't exist in a vertex buffer's uid to buffer range map."));
                return;
            }
        };

        log(&format!("Freed a buffer range: vertices {} -> count {}, indices {} -> count {}",
            occupied_range.get_vertex_offset(),occupied_range.get_vertex_count(),
            occupied_range.get_index_offset(),occupied_range.get_index_count()));

        self.free_buffer_ranges.push(occupied_range);
    }

    //Buffer the data generated by the renderable using the config
    //Assumes this buffer is bound already
    pub fn buffer_data(&mut self, context: &WebGl2RenderingContext, uid: &u32, vertices: &Vec<f32>, indices: &Vec<u32>)
    {
        if self.uid_to_buffer_ranges.contains_key(uid)
        {
            log("Tried to buffer data using a uid that already has data on this buffer.");
            return;
        }

        if T::get_stride() <= 0
        {
            log("No stride.");
            return;
        }

        let mut range_to_occupy : Option<BufferRanges> = None;

        for free_range in &mut self.free_buffer_ranges
        {
            range_to_occupy = free_range.try_split(vertices.len(),indices.len());

            if range_to_occupy.is_some()
            {
                break;
            }
        }

        let range = match range_to_occupy
        {
            Some(r) => r,
            None => {
                log(&format!("Couldn't find a place to put vertex/index data on a vertex buffer. Defaulting to doing nothing."));
                return;
            }
        };

        /*
        //No longer needed, moved to BufferRanges
        let buffer_range = BufferRanges::new(
            Range::<i32>{ start: vertex_offset as i32, end: vertex_offset + vertex_data.len() as i32},
            Range::<i32>{ start: index_offset as i32, end: index_offset + index_data.len() as i32},
        );
        */

        self.do_buffer_data(context,vertices,indices, &range);
        self.uid_to_buffer_ranges.insert(uid.clone(),range);
    }

    fn do_buffer_data(&mut self, context: &WebGl2RenderingContext, vertices: &Vec<f32>, indices: &Vec<u32>, buffer_range: &BufferRanges)
    {
        let vertex_offset_bytes = mem::size_of::<f32>() as i32 * buffer_range.get_vertex_offset();
        let index_offset_bytes = mem::size_of::<u32>() as i32 * buffer_range.get_index_offset();

        let index_data = indices.as_slice();
        let vertex_data = vertices.as_slice();

        //TODO: index_starting_point should not be cast as unsigned - we lose type info. Should be unsigned to begin with....
        //This can panic, we should fix this to be safer.
        let index_starting_point = buffer_range.get_vertex_offset() as u32 / T::get_stride() as u32;

        //Update the indices we are buffering to point to where the vertex data is actually being stored.
        //Indices start out assuming the vertices are at position 0 in the buffer
        let new_indices : Vec<u32> = index_data.iter().map(|&x| x + index_starting_point).collect();

        unsafe
        {
            let vertices_view = Float32Array::view(vertex_data);
            let indices_view = Uint32Array::view(new_indices.as_slice());
            
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                vertex_offset_bytes,
                &vertices_view
            );
            
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                index_offset_bytes,
                &indices_view
            );
        }
    }
}

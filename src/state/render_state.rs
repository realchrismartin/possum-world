use crate::util::logging::log;
use crate::util::logging::log_value;

use crate::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::WebGl2RenderingContext;
use crate::graphics::shader::Shader;
use crate::graphics::sprite::Sprite;
use std::option::Option;
use crate::graphics::vertex_buffer::VertexBuffer;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use crate::graphics::renderable::Renderable;

#[wasm_bindgen]
pub struct RenderState
{
    context: WebGl2RenderingContext,
    shader: Option<Shader>, //TODO: assumes one shader for all buffers
    buffer_map: HashMap<TypeId,Box<dyn Any>>
}

#[wasm_bindgen]
impl RenderState
{
    pub fn new(document : &Document) -> Option<RenderState>
    {
        let canvas = match document.get_element_by_id("canvas")
        {
            Some(canvas) => canvas,
            None => return None
        };
        
        let canvas = match canvas.dyn_into::<web_sys::HtmlCanvasElement>()
        {
            Ok(canvas) => canvas,
            Err(e) => {log_value(&e);return None;}
        };

        let context = match canvas.get_context("webgl2")
        {
            Ok(context) =>context,
            Err(e) => {log_value(&e);return None;}
        };

        let web_context = match context.unwrap().dyn_into::<WebGl2RenderingContext>()
        {
            Ok(context) =>context,
            Err(e) => {log_value(&e);return None;}
        };

        let mut state = Self
        {
            context: web_context,
            shader: None::<Shader>,
            buffer_map: HashMap::new()
        };

        //Init all the buffers here for now because we are using wasm-bindgen and can't put generics in pub fns
        state.init_buffer::<Sprite>();

        Some(state)
    }

    pub fn set_shader(&mut self, vertex_source :&str, frag_source: &str)
    {
        let shader = match Shader::new(&self.context,vertex_source,frag_source)
        {
            Ok(shader) => shader,
            Err(e) => {
                log(e.as_str());
                return;
            }
        };

        self.shader = Some(shader);
    }

    fn init_buffer<T: Renderable + 'static>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        if self.buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return;
        }

        let buffer : VertexBuffer<T> = match VertexBuffer::new(&self.context)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        self.buffer_map.insert(type_id,Box::new(buffer));
    }

    pub fn test_submit_data_and_draw(&mut self)
    {
        if self.shader.is_none()
        {
            return;
        }

        let program = match self.shader.as_ref()
        {
            Some(program) => program,
            None => {return}
        };

        self.context.use_program(Some(program.get_shader_program()));

        //TODO: this is for testing
        let sprite = Sprite::new([
                -0.3,0.3,0.0,
                -0.5,-0.5,0.0,
                0.5,-0.5,0.0,
                0.3,0.3,0.0
            ],[0,1,2,2,3,0]);

        let second_sprite = Sprite::new([
                0.1,0.4,0.0,
                0.2,0.25,0.0,
                0.7,-0.5,0.0,
                0.4,0.3,0.0
            ],[0,1,2,2,3,0]);

        let buffer = match Self::get_mapped_buffer::<Sprite>(&mut self.buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.bind(&self.context);
        buffer.buffer_data(&self.context,&sprite);
        buffer.buffer_data(&self.context,&second_sprite); //TODO: not working in buffer_data

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, buffer.get_index_count() as i32, WebGl2RenderingContext::UNSIGNED_INT,0); //TODO: move context type

        buffer.clear_data();
        VertexBuffer::<Sprite>::unbind(&self.context);
    }

    fn get_mapped_buffer<T: Renderable + 'static>(buffer_map: &mut HashMap<TypeId,Box<dyn Any>>) -> Option<&mut VertexBuffer<T>>
    {
        let type_id = TypeId::of::<T>();

        if !buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = match buffer_map.get_mut(&type_id)
        {
            Some(boxed_buffer) => boxed_buffer,
            None => {return None;}
        };

        return (&mut *boxed_buffer).downcast_mut::<VertexBuffer<T>>()
    }
}
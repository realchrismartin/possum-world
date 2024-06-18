use crate::util::logging::log;
use crate::util::logging::log_value;

use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;
use crate::graphics::shader::Shader;
use crate::graphics::sprite::Sprite;
use crate::graphics::texture::Texture;
use std::option::Option;
use crate::graphics::vertex_buffer::VertexBuffer;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use crate::graphics::renderable::Renderable;
use crate::graphics::camera::Camera;

pub struct RenderState
{
    context: WebGl2RenderingContext,
    shader: Option<Shader>, //TODO: assumes one shader for all buffers
    textures: Vec<Texture>, 
    camera: Camera,
    buffer_map: HashMap<TypeId,Box<dyn Any>>
}

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

        web_context.enable(WebGl2RenderingContext::DEPTH_TEST);

        let mut state = Self
        {
            context: web_context,
            shader: None::<Shader>,
            textures: Vec::<Texture>::new(),
            camera: Camera::new(canvas.width() as f32,canvas.height() as f32),
            buffer_map: HashMap::new()
        };

        //Init all the buffers here for now because we are using wasm-bindgen and can't put generics in pub fns
        state.init_buffer::<Sprite>();

        Some(state)
    }

    //TODO: later move this
    pub fn set_shader(&mut self, vertex_source :&str, frag_source: &str)
    {
        let shader = match Shader::new(&self.context,vertex_source,frag_source)
        {
            Ok(shader) => shader,
            Err(e) => {
                log("Shader error:");
                log(e.as_str());
                return;
            }
        };

        self.context.use_program(Some(shader.get_shader_program()));

        self.shader = Some(shader);
    }

    //TODO: later move this
    pub fn load_texture(&mut self, img: HtmlImageElement)
    {
        let mut the_texture = Texture::new();

        let shader = self.shader.as_ref().expect("No shader bound!");

        let next_texture = WebGl2RenderingContext::TEXTURE0 + (self.textures.len() as u32); //Unsafe? :) TODO

        match the_texture.load(&self.context,img,next_texture)
        {
            Ok(_r) => { },
            Err(e) => {log_value(&e);return;}
        };

        self.textures.push(the_texture);

        //Update the uniform locations
        for i in 0..self.textures.len()
        {
            let uniform_name = format!("u_texture_{}",i);

            let loc =  match self.context.get_uniform_location(shader.get_shader_program(),uniform_name.as_str())
            {
                Some(l) => l,
                None => { 
                    log(format!("No {} uniform exists",uniform_name).as_str());
                    return;
                }
            };

            self.context.uniform1i(Some(&loc), i as i32);
        }

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

    pub fn clear_context(&self)
    {
        let context = &self.context;

        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn submit_data<T: Renderable + 'static>(&mut self,renderable : &T)
    {
        let buffer = match Self::get_mapped_buffer::<T>(&mut self.buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };


        buffer.bind(&self.context);
        buffer.buffer_data(&self.context,&renderable);
        VertexBuffer::<T>::unbind(&self.context);
    }

    fn clear_buffer<T: Renderable + 'static>(&mut self)
    {
        let buffer = match Self::get_mapped_buffer::<T>(&mut self.buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.clear_data();
    }

    pub fn draw_buffer<T: Renderable + 'static>(& self)
    {
        let buffer = match Self::get_const_mapped_buffer::<T>(&self.buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.bind(&self.context);

        self.context.draw_elements_with_i32(buffer.get_draw_type(), buffer.get_index_count() as i32, WebGl2RenderingContext::UNSIGNED_INT,0); //TODO: move context type

        VertexBuffer::<T>::unbind(&self.context);
    }

    fn get_const_mapped_buffer<T: Renderable + 'static>(buffer_map: &HashMap<TypeId,Box<dyn Any>>) -> Option<&VertexBuffer<T>>
    {
        let type_id = TypeId::of::<T>();

        if !buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = match buffer_map.get(&type_id)
        {
            Some(boxed_buffer) => boxed_buffer,
            None => {return None;}
        };

        return (&*boxed_buffer).downcast_ref::<VertexBuffer<T>>()
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

    pub fn submit_transform_uniforms(&self, data : &[f32])
    {
        let shader = match &self.shader 
        {
            Some(shader) => shader,
            None => {return}
        };

        let context = &self.context;
        let m_location = context.get_uniform_location(shader.get_shader_program(),"m_matrices");
        context.uniform_matrix4fv_with_f32_array(m_location.as_ref(),false,data);
    }

    pub fn submit_camera_uniforms(&mut self)
    {
        let shader = match &self.shader 
        {
            Some(shader) => shader,
            None => {return}
        };

        let camera = &mut self.camera;
        let context = &self.context;

        if !camera.dirty()
        {
           return; 
        }

        camera.recalculate();

        let vp_location = context.get_uniform_location(shader.get_shader_program(),"vp_matrix");

        let view_projection_matrix = camera.get_view_projection_matrix();
        
        let vp_converted : glm::Mat4 = view_projection_matrix.into();

        context.uniform_matrix4fv_with_f32_array(vp_location.as_ref(),false,vp_converted.as_slice());
    }
}
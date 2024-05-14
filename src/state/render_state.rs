use crate::util::logging::log;
use crate::util::logging::log_value;

use crate::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use crate::graphics::shader::Shader;
use crate::graphics::sprite::Sprite;
use std::option::Option;
use crate::graphics::vertex_buffer::VertexBuffer;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use crate::graphics::renderable::Renderable;
use crate::graphics::camera::Camera;

#[wasm_bindgen]
pub struct RenderState
{
    context: WebGl2RenderingContext,
    shader: Option<Shader>, //TODO: assumes one shader for all buffers
    camera: Camera,
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
            camera: Camera::new(canvas.width() as f32,canvas.height() as f32),
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

        //TODO: later move this
        self.context.use_program(Some(shader.get_shader_program()));

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

    //TODO: this is for testing
    pub fn test_submit_sprite_data(&mut self)
    {
        //Add the transform data
        let s_w: glm::Mat4 = glm::Mat4::identity().into();
        let mut s_2_w: glm::Mat4= glm::Mat4::identity().into();
        
        let translation : glm::TVec3<f32> = glm::vec3(0.5,0.5,0.0);
        s_2_w = glm::translate(&s_2_w,&translation);

        let mut transform_uniform_data= Vec::<f32>::new();

        transform_uniform_data.extend_from_slice(s_w.as_slice());
        transform_uniform_data.extend_from_slice(s_2_w.as_slice());

        //Add the renderable stuff
        let sprite = Sprite::new([
                -0.5,0.5,0.0,
                0.0,
                -0.5,-0.5,0.0,
                0.0,
                0.5,-0.5,0.0,
                0.0,
                0.5,0.5,0.0,
                0.0
            ],[0,1,2,2,3,0]);

        let second_sprite = Sprite::new([
                0.1,0.4,0.0,
                1.0,
                0.2,0.25,0.0,
                1.0,
                0.7,-0.5,0.0,
                1.0,
                0.4,0.3,0.0,
                1.0
            ],[0,1,2,2,3,0]);

        self.submit_camera_uniforms();
        self.submit_transform_uniforms(&transform_uniform_data);
        self.submit_data(&sprite);
        self.submit_data(&second_sprite);
    }

    pub fn test_draw_sprites(&mut self)
    {
        self.clear_context();
        self.draw_buffer::<Sprite>();
        self.clear_buffer::<Sprite>();
    }

    fn clear_context(&self)
    {
        let context = &self.context;

        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    fn submit_data<T: Renderable + 'static>(&mut self,renderable : &T)
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

    fn draw_buffer<T: Renderable + 'static>(&mut self)
    {
        let buffer = match Self::get_mapped_buffer::<T>(&mut self.buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.bind(&self.context);

        self.context.draw_elements_with_i32(buffer.get_draw_type(), buffer.get_index_count() as i32, WebGl2RenderingContext::UNSIGNED_INT,0); //TODO: move context type

        VertexBuffer::<T>::unbind(&self.context);
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

    fn submit_transform_uniforms(&self, data : &[f32])
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

    fn submit_camera_uniforms(&mut self)
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
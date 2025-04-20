use crate::util::logging::log;
use crate::util::logging::log_value;

use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;
use crate::graphics::shader::Shader;
use crate::graphics::texture::Texture;
use std::option::Option;
use crate::graphics::vertex_buffer::VertexBuffer;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use crate::graphics::renderable::{Renderable,RenderableConfig};
use crate::graphics::camera::Camera;
use crate::graphics::transform_buffer::TransformBuffer;
use crate::util::util::{world_position_to_screen_translation,screen_translation_to_world_position};
use crate::graphics::draw_batch::DrawBatch;

pub struct RenderState
{
    context: WebGl2RenderingContext,
    shader: Option<Shader>, //TODO: assumes one shader for all buffers
    textures: HashMap<u32,Texture>, 
    camera: Camera,
    vertex_buffer_map: HashMap<TypeId,Box<dyn Any>>,
    transform_buffer: TransformBuffer,
    next_uid: u32
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

        let transform_buffer = match TransformBuffer::new(&web_context,"ModelMatrixBlock")
        {
            Some(buffer) => buffer,
            None => { return None; }
        };

        web_context.enable(WebGl2RenderingContext::BLEND);
        web_context.blend_func(WebGl2RenderingContext::SRC_ALPHA ,WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        web_context.enable(WebGl2RenderingContext::DEPTH_TEST);
        web_context.clear_color(0.0, 0.0, 0.0, 1.0);
        web_context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        Some(Self
        {
            context: web_context,
            shader: None::<Shader>,
            textures: HashMap::new(),
            camera: Camera::new(canvas.width(),canvas.height()),
            vertex_buffer_map: HashMap::new(),
            transform_buffer: transform_buffer,
            next_uid: 0
        })
    }

    pub fn request_new_transform(&mut self) -> u32
    {
        self.transform_buffer.request_new_transform()
    }
    pub fn request_new_renderable_with_existing_transform<T: Renderable + 'static>(&mut self, renderable_config: &RenderableConfig, existing_transform: u32) -> Option<T>
    {
        self.request_new_renderable_impl(renderable_config,Some(existing_transform))
    }

    pub fn request_new_renderable<T: Renderable + 'static>(&mut self, renderable_config: &RenderableConfig) -> Option<T>
    {
        self.request_new_renderable_impl(renderable_config,None)
    }

    fn request_new_renderable_impl<T: Renderable + 'static>(&mut self, renderable_config: &RenderableConfig, existing_transform: Option<u32>) -> Option<T>
    {
        let texture_dimensions = match self.get_texture(renderable_config.get_texture_index())
        {
            Some(t) => t.get_dimensions(),
            None => { return None; }
        };

        let world_size_x = self.get_world_size_x(); //In pixels
        let world_size_y = self.get_world_size_y();
        let world_size = [world_size_x as f32, world_size_y as f32];

        //Copy the RC to make it mutable
        let mut copied_renderable_config = renderable_config.clone();

        //Set mutable properties
        copied_renderable_config.set_texture_dimensions(&texture_dimensions);
        copied_renderable_config.set_world_size_ratio(&world_size);

        //If an existing transform is requested, use it, otherwise:
        //Request a transform from the buffer. It lives there in RAM. The buffer will handle moving the data over to uniforms.
        let transform_location = match existing_transform
        {
            Some(transform) => transform,
            None => self.transform_buffer.request_new_transform()
        };

        //Create a renderable
        let mut renderable = T::new(self.next_uid,transform_location,*copied_renderable_config.get_size());

        //immediately submit its data to the buffer. This will only be done once.
        self.submit_data(&renderable,&copied_renderable_config);
        self.next_uid += 1;

        Some(renderable)
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

    pub fn load_texture(&mut self, index: u32, img: HtmlImageElement)
    {
        let mut the_texture = Texture::new();

        let shader = self.shader.as_ref().expect("No shader bound!");

        let next_texture = WebGl2RenderingContext::TEXTURE0 + index;

        let uniform_name = format!("u_texture_{}",index);

        let loc =  match self.context.get_uniform_location(shader.get_shader_program(),uniform_name.as_str())
        {
            Some(l) => l,
            None => { 
                log(format!("No {} uniform exists",uniform_name).as_str());
                return;
            }
        };

        match the_texture.load(&self.context,img,next_texture)
        {
            Ok(_r) => { },
            Err(e) => {log_value(&e);return;}
        };

        self.textures.insert(index,the_texture);
        self.context.uniform1i(Some(&loc), index as i32);

    }

    pub fn clear_context(&self)
    {
        let _ = &self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn set_position_with_index(&mut self, transform_index: u32, position : glm::Vec3)
    {
        self.set_translation_with_index(transform_index, world_position_to_screen_translation(&position,
            &glm::vec2(self.camera.get_canvas_width() as f32, self.camera.get_canvas_height() as f32)));
    }

    pub fn get_position_with_index(&self, transform_index: u32) -> Option<glm::Vec3>
    {
        let translation = match self.transform_buffer.get_translation(transform_index)
        {
            Some(t) => t,
            None => {return None;}
        };

        Some(screen_translation_to_world_position(&translation,
            &glm::vec2(self.camera.get_canvas_width() as f32, self.camera.get_canvas_height() as f32)))
    }

    fn set_translation_with_index(&mut self, transform_index: u32, translation: glm::Vec3)
    {
        self.transform_buffer.set_translation(transform_index, translation);
    }

    pub fn set_rotation_with_index(&mut self, transform_index: u32, rotation: f32)
    {
        self.transform_buffer.set_rotation(transform_index, rotation);
    }

    pub fn set_scale_with_index(&mut self, transform_index: u32, scale: glm::Vec3)
    {
        self.transform_buffer.set_scale(transform_index,scale);
    }

    pub fn get_scale_with_index(&self, transform_index: u32) -> Option<&glm::Vec3>
    {
        let scale = match self.transform_buffer.get_scale(transform_index)
        {
            Some(t) => t,
            None => {return None;}
        };

        Some(scale)
    }

    //0,0 is the bottom left corner of the world
    //0,max_y is the top left corner
    pub fn set_position<T: Renderable + 'static>(&mut self, renderable: &T, position : glm::Vec3)
    {
        self.transform_buffer.set_translation(renderable.get_transform_location(), world_position_to_screen_translation(&position,
            &glm::vec2(self.camera.get_canvas_width() as f32, self.camera.get_canvas_height() as f32)));
    }

    pub fn set_rotation<T: Renderable + 'static>(&mut self, renderable: &T, rotation: f32)
    {
        self.transform_buffer.set_rotation(renderable.get_transform_location(), rotation);
    }

    pub fn set_scale<T: Renderable + 'static>(&mut self, renderable: &T, scale: glm::Vec3)
    {
        self.transform_buffer.set_scale(renderable.get_transform_location(),scale);
    }

    pub fn bind_and_update_transform_buffer_data(&mut self)
    {
        let shader = match &self.shader 
        {
            Some(shader) => shader,
            None => {return}
        };

        //Bind the UBO to the shader before rendering
        self.transform_buffer.bind_to_shader(&self.context, shader);

        //Recalculate matrices that are marked dirty and need recalculating.
        //Upload any matrices to the UBO that have changed.
        self.transform_buffer.recalculate_transforms_and_update_data(&self.context);
    }

    pub fn submit_camera_uniforms(&mut self)
    {
        if !self.camera.dirty()
        {
           return; 
        }

        let shader = match &self.shader 
        {
            Some(shader) => shader,
            None => {return}
        };

        let camera = &mut self.camera;
        let context = &self.context;

        camera.recalculate();

        let vp_location = context.get_uniform_location(shader.get_shader_program(),"vp_matrix");

        let view_projection_matrix = camera.get_view_projection_matrix();
        
        let vp_converted : glm::Mat4 = view_projection_matrix.into();

        context.uniform_matrix4fv_with_f32_array(vp_location.as_ref(),false,vp_converted.as_slice());
    }

    pub fn draw<T: Renderable + 'static>(&self, draw_batch: &DrawBatch<T>)
    {
        let buffer = match Self::get_const_mapped_buffer::<T>(&self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.bind(&self.context);

        for uid in draw_batch.get_uids()
        {
            let range = match buffer.get_draw_range_for_uid(&uid)
            {
                Some(r) => r,
                None => {continue;}
            };

            let count = range.end - range.start;

            if count < 0 
            {
                //NB: already checked by buffer
                continue;
            }

            self.context.draw_elements_with_i32(T::get_draw_type(),count, WebGl2RenderingContext::UNSIGNED_INT,range.start);
        }

        VertexBuffer::<T>::unbind(&self.context);
    }

    fn get_texture(&self, index: u32) -> Option<&Texture>
    {
        if !self.textures.contains_key(&index)
        {
            return None;
        }

        self.textures.get(&index)
    }

    fn init_buffer<T: Renderable + 'static>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        if self.vertex_buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return;
        }

        let buffer : VertexBuffer<T> = match VertexBuffer::new(&self.context)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        self.vertex_buffer_map.insert(type_id,Box::new(buffer));
    }

    fn submit_data<T: Renderable + 'static>(&mut self,renderable : &T, renderable_config: &RenderableConfig)
    {
        let type_id = TypeId::of::<T>();
        if !self.vertex_buffer_map.contains_key(&type_id)
        {
            //Lazily initialize our buffer here.
            self.init_buffer::<T>();
        }

        //Now that we've perhaps lazily initialized, grab a ref to the buffer.
        let buffer = match Self::get_mapped_buffer::<T>(&mut self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => { return; }
        };

        buffer.bind(&self.context);
        buffer.buffer_data(&self.context,&renderable,&renderable_config);
        VertexBuffer::<T>::unbind(&self.context);
    }

    fn get_const_mapped_buffer<T: Renderable + 'static>(vertex_buffer_map: &HashMap<TypeId,Box<dyn Any>>) -> Option<&VertexBuffer<T>>
    {
        let type_id = TypeId::of::<T>();

        if !vertex_buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = match vertex_buffer_map.get(&type_id)
        {
            Some(boxed_buffer) => boxed_buffer,
            None => {return None;}
        };

        return (&*boxed_buffer).downcast_ref::<VertexBuffer<T>>()
    }

    fn get_mapped_buffer<T: Renderable + 'static>(vertex_buffer_map: &mut HashMap<TypeId,Box<dyn Any>>) -> Option<&mut VertexBuffer<T>>
    {
        let type_id = TypeId::of::<T>();

        if !vertex_buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = match vertex_buffer_map.get_mut(&type_id)
        {
            Some(boxed_buffer) => boxed_buffer,
            None => {return None;}
        };

        return (&mut *boxed_buffer).downcast_mut::<VertexBuffer<T>>()
    }

    pub fn set_canvas_dimensions(&mut self, x: u32, y: u32)
    {
        self.context.viewport(0, 0, x as i32, y as i32);

        let viewport = self.context.get_parameter(WebGl2RenderingContext::VIEWPORT).unwrap();
        log_value(&viewport);

        self.camera.set_canvas_dimensions(x, y);
    }

    pub fn get_world_size_x(&self) -> u32
    {
        self.camera.get_canvas_width()
    }

    pub fn get_world_size_y(&self) -> u32
    {
        self.camera.get_canvas_height()
    }

    pub fn clear_buffer<T: Renderable + 'static>(&mut self)
    {
        let buffer = match Self::get_mapped_buffer::<T>(&mut self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.clear();
    }

    pub fn clear_transform_buffer(&mut self)
    {
        self.transform_buffer.clear();
    }

}
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
use crate::util::util::world_position_to_screen_translation;
use std::ops::Range;

static WORLD_SIZE_X : f32 = 1000.0;
static WORLD_SIZE_Y : f32 = 1000.0;

pub struct RenderState
{
    context: WebGl2RenderingContext,
    shader: Option<Shader>, //TODO: assumes one shader for all buffers
    textures: HashMap<u32,Texture>, 
    camera: Camera,
    vertex_buffer_map: HashMap<TypeId,Box<dyn Any>>,
    transform_buffer: TransformBuffer
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

        //TODO: fix blending
        //web_context.enable(WebGl2RenderingContext::BLEND);
        //web_context.blend_func(WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,WebGl2RenderingContext::DST_COLOR);
        web_context.enable(WebGl2RenderingContext::DEPTH_TEST);

        Some(Self
        {
            context: web_context,
            shader: None::<Shader>,
            textures: HashMap::new(),
            camera: Camera::new(canvas.width() as f32,canvas.height() as f32),
            vertex_buffer_map: HashMap::new(),
            transform_buffer: TransformBuffer::new()
        })
    }

    pub fn request_new_renderable<T: Renderable + 'static>(&mut self, renderable_config: &RenderableConfig) -> Option<T>
    {
        let texture_dimensions = match self.get_texture(renderable_config.get_texture_index())
        {
            Some(t) => t.get_dimensions(),
            None => { return None; }
        };

        //Get the space on the buffer that the new data will take up.
        //NB: this relies on the property that any given instance of a specific Renderable always has the same # of indices.
        let element_location = match self.get_next_range_on_buffer::<T>()
        {
            Some(range) => range,
            None => { return None; }
        };

        //Copy the RC to make it mutable
        let mut copied_renderable_config = renderable_config.clone();
        copied_renderable_config.set_texture_dimensions(&texture_dimensions);

        //Request a transform from the buffer. It lives there in RAM. The buffer will handle moving the data over to uniforms.
        let transform_location = self.transform_buffer.request_new_transform();

        //Create a renderable
        let renderable = T::new(element_location, transform_location);

        //immediately submit its data to the buffer. This will only be done once.
        self.submit_data(&renderable,&copied_renderable_config);

        //Return the renderable. The owner of the renderable can later submit it to be drawn, so we will know which buffer to use.
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
        let context = &self.context;

        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    //0,0 is the bottom left corner of the world
    //0,max_y is the top left corner
    pub fn set_position<T: Renderable + 'static>(&mut self, renderable: &T, position : glm::Vec2)
    {
        self.set_translation(renderable, world_position_to_screen_translation(&position,&glm::vec2(WORLD_SIZE_X,WORLD_SIZE_Y)));
    }

    pub fn set_translation<T: Renderable + 'static>(&mut self, renderable: &T, translation: glm::Vec3)
    {
        self.transform_buffer.set_translation(renderable.get_transform_location(), translation);
    }

    pub fn set_rotation<T: Renderable + 'static>(&mut self, renderable: &T, rotation: f32)
    {
        self.transform_buffer.set_rotation(renderable.get_transform_location(), rotation);
    }

    pub fn set_scale<T: Renderable + 'static>(&mut self, renderable: &T, scale: glm::Vec3)
    {
        self.transform_buffer.set_scale(renderable.get_transform_location(),scale);
    }

    pub fn submit_transform_buffer_uniforms(&mut self)
    {
        //For any of the transforms that need to be recalculated, do it now.
        //TODO: later, optimize so that we don't have to iterate over all transforms here.
        self.transform_buffer.recalculate_transforms_and_update_data();

        //If the recalculation didn't require the buffer to change, do nothing.
        if !self.transform_buffer.dirty()
        {
            return;
        }

        let shader = match &self.shader 
        {
            Some(shader) => shader,
            None => {return}
        };

        //Unfortunately, we need to update all of the transform data if any one of the matrices changes (buffer becomes dirty)
        //Optimize this later if we can.
        let context = &self.context;
        let m_location = context.get_uniform_location(shader.get_shader_program(),"m_matrices");
        context.uniform_matrix4fv_with_f32_array(m_location.as_ref(),false,&self.transform_buffer.data().as_slice());

        log(format!("This many uniforms uploaded: {}",&self.transform_buffer.data().len()).as_str());

        self.transform_buffer.set_clean();
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

    pub fn draw<T: Renderable + 'static>(&self, renderables: &Vec<T>)
    {
        let buffer = match Self::get_const_mapped_buffer::<T>(&self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.bind(&self.context);

        for renderable in renderables
        {
            let range = renderable.get_element_location();

            let count = range.end - range.start;

            if count < 0 
            {
                continue;
            }

            if !buffer.is_range_valid(&range)
            {
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

    fn get_next_range_on_buffer<T: Renderable + 'static>(&mut self) -> Option<Range<i32>>
    {
        let type_id = TypeId::of::<T>();
        if !self.vertex_buffer_map.contains_key(&type_id)
        {
            //Lazily initialize our buffer here.
            self.init_buffer::<T>();
        }

        let buffer = match Self::get_const_mapped_buffer::<T>(&mut self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => {return None; }
        };

        Some(buffer.get_next_range())
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
            None => {return; }
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
}
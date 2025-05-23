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
use crate::graphics::renderable::Renderable;
use crate::graphics::camera::Camera;
use crate::graphics::transform_buffer::TransformBuffer;
use crate::graphics::draw_batch::DrawBatch;
use std::cmp::Ordering;

pub struct RenderState
{
    context: Option<WebGl2RenderingContext>,
    shader: Option<Shader>, //TODO: assumes one shader for all buffers
    textures: HashMap<u32,Texture>, 
    camera: Camera,
    vertex_buffer_map: HashMap<TypeId,Box<dyn Any>>,
    transform_buffer: TransformBuffer,
    next_uid: u32
}

impl RenderState
{
    pub fn new(document : &Document) -> RenderState
    {
        let web_context = Self::get_context_for_document(document);
        let canvas_size = Self::get_canvas_size(document);
        let transform_buffer = TransformBuffer::new(web_context.as_ref(),"ModelMatrixBlock");

        Self
        {
            context: web_context,
            shader: None::<Shader>,
            textures: HashMap::new(),
            camera: Camera::new(canvas_size[0],canvas_size[1]),
            vertex_buffer_map: HashMap::new(),
            transform_buffer: transform_buffer,
            next_uid: 0
        }
    }

    fn get_canvas_size(document: &Document) -> [u32;2]
    {
        let canvas = match document.get_element_by_id("canvas")
        {
            Some(canvas) => canvas,
            None => {return [1,1];}
        };
        
        let canvas = match canvas.dyn_into::<web_sys::HtmlCanvasElement>()
        {
            Ok(canvas) => canvas,
            Err(_) => {return [1,1];}
        };

        return [canvas.width(),canvas.height()];
    }

    fn get_context_for_document(document: &Document) -> Option<WebGl2RenderingContext>
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

        web_context.enable(WebGl2RenderingContext::BLEND);
        web_context.blend_func(WebGl2RenderingContext::SRC_ALPHA ,WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        web_context.enable(WebGl2RenderingContext::DEPTH_TEST);
        web_context.disable(WebGl2RenderingContext::CULL_FACE);
        web_context.clear_color(0.0, 0.0, 0.0, 1.0);
        web_context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        Some(web_context)
    }

    pub fn free_renderable<T: Renderable>(&mut self, renderable: &T)
    {
        let type_id = TypeId::of::<T>();
        if !self.vertex_buffer_map.contains_key(&type_id)
        {
            return;
        }

        let buffer = match Self::get_mut_mapped_buffer::<T>(&mut self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => { return; }
        };

        
        buffer.free(&renderable.get_renderable_uid());

        self.transform_buffer.free_transform_if_no_longer_referenced(&renderable.get_renderable_uid());
    }

    pub fn request_new_renderable_with_existing_transform<T: Renderable>(&mut self, renderable: &mut T, reuse_existing_transform_for_uid: u32)
    {
        self.request_new_renderable_impl::<T>(renderable,&Some(reuse_existing_transform_for_uid))
    }

    pub fn request_new_renderable<T: Renderable>(&mut self, renderable: &mut T)
    {
        self.request_new_renderable_impl::<T>(renderable,&None::<u32>)
    }

    fn request_new_renderable_impl<T: Renderable>(&mut self, renderable: &mut T, reuse_existing_transform_for_uid: &Option<u32>)
    {
        self.next_uid += 1;

        let new_uid = self.next_uid.clone();

        //If an existing transform is requested, use it, otherwise:
        //Request a transform from the buffer. It lives there in RAM. The buffer will handle moving the data over to uniforms.
        let model_matrix_transform_index = match reuse_existing_transform_for_uid
        {
            Some(t) => {
                //An existing UID was provided, use this UID to find an existing transform index to use.
                //The buffer will mark that this transform is being used by both uids (plus any that were already using it).
                self.transform_buffer.reuse_existing_transform_index(&new_uid,&t)
            },
            None => {
                //No UID was provided - we need a new transform. Use the new UID to generate it.
                //The buffer will mark that this transform is being used by this uid.
                self.transform_buffer.request_new_transform_index(&new_uid)
            }
        };

        //immediately submit data to the buffer. This will only be done once.
        self.submit_data::<T>(&new_uid, &renderable.get_vertices(&self, model_matrix_transform_index), &renderable.get_indices());

        renderable.set_renderable_uid(new_uid);
        
        match renderable.get_starting_world_position()
        {
            Some(p) => { self.set_position(&new_uid, p)},
            None => {}
        };

        match renderable.get_starting_z()
        {
            Some(z) => { self.set_z(&new_uid, z)},
            None => {}
        };

        match renderable.get_starting_scale()
        {
            Some(s) => { self.set_scale(&new_uid, s)},
            None => {}
        };
    }

    //TODO: later move this
    pub fn set_shader(&mut self, vertex_source :&str, frag_source: &str)
    {
        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        let shader = match Shader::new(web_context,vertex_source,frag_source)
        {
            Ok(shader) => shader,
            Err(e) => {
                log("Shader error:");
                log(e.as_str());
                return;
            }
        };

        web_context.use_program(Some(shader.get_shader_program()));

        self.shader = Some(shader);
    }

    pub fn load_texture(&mut self, index: u32, img: HtmlImageElement)
    {
        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        let mut the_texture = Texture::new();

        let shader = self.shader.as_ref().expect("No shader bound!");

        let next_texture = WebGl2RenderingContext::TEXTURE0 + index;

        let uniform_name = format!("u_texture_{}",index);

        let loc =  match web_context.get_uniform_location(shader.get_shader_program(),uniform_name.as_str())
        {
            Some(l) => l,
            None => { 
                log(format!("No {} uniform exists",uniform_name).as_str());
                return;
            }
        };

        match the_texture.load(web_context,img,next_texture)
        {
            Ok(_r) => { },
            Err(e) => {log_value(&e);return;}
        };

        self.textures.insert(index,the_texture);
        web_context.uniform1i(Some(&loc), index as i32);

    }

    pub fn clear_context(&self)
    {
        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        let _ = web_context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn set_position(&mut self, uid: &u32, position: &glm::Vec2)
    {
        self.transform_buffer.set_translation(uid,&position);
    }

    pub fn set_z(&mut self, uid: &u32, z: f32)
    {
        self.transform_buffer.set_z(uid,z);
    }

    fn get_z(&self, uid: &u32) -> Option<&f32>
    {
        self.transform_buffer.get_z(uid)
    }

    pub fn set_rotation(&mut self, uid: &u32, rotation: f32)
    {
        self.transform_buffer.set_rotation(uid, rotation);
    }

    pub fn set_scale(&mut self, uid: &u32, scale: &glm::Vec2)
    {
        self.transform_buffer.set_scale(uid,scale)
    }

    pub fn bind_and_update_transform_buffer_data(&mut self)
    {
        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };


        let shader = match self.shader.as_ref()
        {
            Some(shader) => shader,
            None => {return}
        };

        //Bind the UBO to the shader before rendering
        self.transform_buffer.bind_to_shader(web_context, shader);

        //Recalculate matrices that are marked dirty and need recalculating.
        //Upload any matrices to the UBO that have changed.
        self.transform_buffer.recalculate_transforms_and_update_data(web_context);
    }

    pub fn set_camera_world_position(&mut self, position: &glm::Vec2)
    {
        self.camera.set_camera_world_position(position);
    }

    pub fn submit_camera_uniforms(&mut self)
    {
        if !self.camera.dirty()
        {
           return; 
        }

        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        let shader = match &self.shader 
        {
            Some(shader) => shader,
            None => {return}
        };

        let camera = &mut self.camera;

        camera.recalculate();

        let vp_location = web_context.get_uniform_location(shader.get_shader_program(),"vp_matrix");

        let view_projection_matrix = camera.get_view_projection_matrix();
        
        let vp_converted : glm::Mat4 = view_projection_matrix.into();

        web_context.uniform_matrix4fv_with_f32_array(vp_location.as_ref(),false,vp_converted.as_slice());
    }

    pub fn draw<T: Renderable>(&self, draw_batch: &mut DrawBatch<T>)
    {
        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        let buffer = match Self::get_mapped_buffer::<T>(&self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.bind(web_context);
        
        //Sort by Z so that we sample transparency correctly
        //TODO: can we stop doing this?
        //NB: this will NOT work across batches.
        draw_batch.get_mut_uids().sort_by(|a, b| 
        {
            let a_z = match self.get_z(a)
            {
                Some(z) => *z,
                None => {return Ordering::Equal;}
            };

           let b_z = match self.get_z(b)
            {
                Some(z) => *z,
                None => {return Ordering::Equal;}
            };

            if a_z < b_z
            {
                return Ordering::Less;
            }

            Ordering::Greater
        });

        //TODO: could set state on the buffer about which elements to draw next tick instead of making a batch
        for uid in draw_batch.get_mut_uids()
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

            //TODO: could pass this into buffer to remove need to pass range back here
            web_context.draw_elements_with_i32(T::get_draw_type(),count, WebGl2RenderingContext::UNSIGNED_INT,range.start);
        }

        VertexBuffer::<T>::unbind(web_context);
    }

    pub fn get_texture(&self, index: u32) -> Option<&Texture>
    {
        if !self.textures.contains_key(&index)
        {
            return None;
        }

        self.textures.get(&index)
    }

    fn init_buffer<T: Renderable>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        if self.vertex_buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return;
        }

        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        let buffer : VertexBuffer<T> = match VertexBuffer::new(web_context)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        self.vertex_buffer_map.insert(type_id,Box::new(buffer));
    }

    fn submit_data<T: Renderable>(&mut self,uid: &u32, vertices: &Vec<f32>, indices: &Vec<u32>)
    {
        let type_id = TypeId::of::<T>();
        if !self.vertex_buffer_map.contains_key(&type_id)
        {
            //Lazily initialize our buffer here.
            self.init_buffer::<T>();
        }

        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        //Now that we've perhaps lazily initialized, grab a ref to the buffer.
        let buffer = match Self::get_mut_mapped_buffer::<T>(&mut self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => { return; }
        };

        buffer.bind(web_context);
        buffer.buffer_data(web_context,uid,vertices,indices);
        VertexBuffer::<T>::unbind(web_context);
    }

    fn get_mapped_buffer<T: Renderable>(vertex_buffer_map: &HashMap<TypeId,Box<dyn Any>>) -> Option<&VertexBuffer<T>>
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

        boxed_buffer.downcast_ref::<VertexBuffer<T>>()
    }

    fn get_mut_mapped_buffer<T: Renderable>(vertex_buffer_map: &mut HashMap<TypeId,Box<dyn Any>>) -> Option<&mut VertexBuffer<T>>
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

        boxed_buffer.downcast_mut::<VertexBuffer<T>>()
    }

    pub fn set_canvas_dimensions(&mut self, x: u32, y: u32)
    {
        let web_context = match self.context.as_ref()
        {
            Some(c) => c,
            None => { return; }
        };

        web_context.viewport(0, 0, x as i32, y as i32);

        let viewport = web_context.get_parameter(WebGl2RenderingContext::VIEWPORT).unwrap();

        self.camera.set_canvas_dimensions(x, y);
        
        //Zoom the camera to an appropriate level based on how big the canvas is.
        //900 is an arbitrary value for a decent zoom
        self.camera.set_zoom(900.0 / std::cmp::min(x,y) as f32);
    }

    pub fn clear(&mut self)
    {
        self.next_uid = 0;
        self.transform_buffer.clear();
    }

    pub fn clear_buffer<T: Renderable>(&mut self)
    {
        let buffer = match Self::get_mut_mapped_buffer::<T>(&mut self.vertex_buffer_map)
        {
            Some(buffer) => buffer,
            None => {return}
        };

        buffer.clear();
    }
}

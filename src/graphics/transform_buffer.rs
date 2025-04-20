use web_sys::{WebGl2RenderingContext,WebGlBuffer};
use crate::graphics::transform::Transform;
use crate::graphics::shader::Shader;
use std::collections::HashSet;
use std::collections::HashMap;
use web_sys::js_sys::Float32Array;
use std::mem;
use crate::util::logging::log;

static MAX_BUFFER_SIZE : usize = 16384; //1024 4x4 Matrices

pub struct TransformBuffer
{
    uniform_name: String,
    transforms: Vec<Transform>,
    ubo: WebGlBuffer,
    next_available_index: u32,
    dirty_transforms: HashSet<u32>,
    uid_to_index_map: HashMap<u32,u32>
}

impl TransformBuffer
{
    pub fn new(context: &WebGl2RenderingContext, uniform_name: &str) -> Option<Self>
    {
        let buffer = match context.create_buffer()
        {
            Some(b) => b,
            None => {return None;}
        };

        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER,Some(&buffer));
        context.buffer_data_with_i32(WebGl2RenderingContext::UNIFORM_BUFFER, (MAX_BUFFER_SIZE * std::mem::size_of::<f32>()) as i32, WebGl2RenderingContext::STATIC_DRAW);
        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER,None);

        let max_matrices = context.get_parameter(WebGl2RenderingContext::MAX_UNIFORM_BLOCK_SIZE).unwrap().as_f64().unwrap() / mem::size_of::<glm::Mat4>() as f64;

        log(format!("Max Matrices in UBO: {}",max_matrices).as_str());

        Some(Self {
            uniform_name: uniform_name.to_string(),
            transforms: Vec::new(),
            ubo: buffer,
            next_available_index: 0,
            dirty_transforms: HashSet::new(),
            uid_to_index_map: HashMap::new()
        })
    }

    pub fn bind_to_shader(&self, context: &WebGl2RenderingContext, shader: &Shader)
    {
        let block_index = context.get_uniform_block_index(shader.get_shader_program(), self.uniform_name.as_str());
        context.uniform_block_binding(shader.get_shader_program(), block_index, 0);
        context.bind_buffer_base(WebGl2RenderingContext::UNIFORM_BUFFER, 0, Some(&self.ubo));
    }

    pub fn unbind_from_shader(&self, context:&WebGl2RenderingContext)
    {
        //TODO: works?
        context.bind_buffer_base(WebGl2RenderingContext::UNIFORM_BUFFER, 0, None);
    }

    pub fn bind(&self, context: &WebGl2RenderingContext)
    {        
       context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, Some(&self.ubo));
    }

    pub fn unbind(context: &WebGl2RenderingContext)
    {
       context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, None);
    }

    pub fn set_translation(&mut self, uid: &u32, translation: glm::Vec3)
    {
        let index = match self.uid_to_index_map.get(uid)
        {
            Some(i) => i,
            None => { return; }
        };

        if self.transforms.len() <= *index as usize
        {
            return;
        }        

        self.transforms[*index as usize].set_translation(translation);

        self.dirty_transforms.insert(*index);
    }

    pub fn get_translation(&self, uid: &u32) -> Option<&glm::Vec3>
    {
        let index = match self.uid_to_index_map.get(uid)
        {
            Some(i) => i,
            None => { return None; }
        };

        if self.transforms.len() <= *index as usize
        {
            return None;
        }        

        Some(self.transforms[*index as usize].get_translation())
    }

    pub fn set_rotation(&mut self, uid: &u32, rotation: f32)
    {
        let index = match self.uid_to_index_map.get(uid)
        {
            Some(i) => i,
            None => { return; }
        };


        if self.transforms.len() <= *index as usize
        {
            return;
        }       
        self.transforms[*index as usize].set_rotation(rotation);

        self.dirty_transforms.insert(index.clone());
    }

    pub fn set_scale(&mut self, uid: &u32, scale: glm::Vec3)
    {
        let index = match self.uid_to_index_map.get(uid)
        {
            Some(i) => i,
            None => { return; }
        };


        if self.transforms.len() <= *index as usize
        {
            return;
        }       

        self.transforms[*index as usize].set_scale(scale);

        self.dirty_transforms.insert(index.clone());
    }

    pub fn get_scale(&self, uid: &u32) -> Option<&glm::Vec3>
    {
        let index = match self.uid_to_index_map.get(uid)
        {
            Some(i) => i,
            None => { return None; }
        };

        if self.transforms.len() <= *index as usize
        {
            return None;
        }        

        Some(self.transforms[*index as usize].get_scale())
    }

    //For each transform matrix, update the raw data if it needs to be updated.
    pub fn recalculate_transforms_and_update_data(&mut self, context: &WebGl2RenderingContext)
    {
        for dirty_transform_index in &self.dirty_transforms
        {
            if *dirty_transform_index >= self.transforms.len() as u32
            {
                continue;
            }

            let transform = match self.transforms.get(*dirty_transform_index as usize)
            {
                Some(t) => t,
                None => { continue; }
            };

            let matrix = transform.calculate();
            let offset = mem::size_of::<f32>() * 16 * (*dirty_transform_index as usize);

            unsafe
            {
                let transform_data_view = Float32Array::view(matrix.as_slice());
                
                context.buffer_sub_data_with_i32_and_array_buffer_view(
                    WebGl2RenderingContext::UNIFORM_BUFFER,
                    offset as i32,
                    &transform_data_view
                );
            }
        }

        self.dirty_transforms.clear();
    }

    pub fn reuse_existing_transform_index(&mut self, uid: &u32, uid_to_reuse_transform_from: &u32) -> u32 
    {
        let index = match self.uid_to_index_map.get(uid_to_reuse_transform_from)
        {
            Some(i) => *i,
            None => { 
                self.request_new_transform_index(uid)
            }
        };

        self.uid_to_index_map.insert(uid.clone(),index.clone());

        index.clone()
    }

    pub fn request_new_transform_index(&mut self, uid: &u32) -> u32 
    {
        //UID isn't in map, make a new transform
        let mat_index = self.next_available_index;
        self.next_available_index += 1;

        self.transforms.push(Transform::new());
        self.dirty_transforms.insert(mat_index);
        self.uid_to_index_map.insert(uid.clone(),mat_index);

        mat_index
    }

    pub fn clear(&mut self)
    {
        //TODO: clear data or don't
        self.next_available_index = 0;
        self.dirty_transforms.clear();
        self.transforms.clear();
        self.uid_to_index_map.clear();
    }
}
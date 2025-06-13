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
    ubo: Option<WebGlBuffer>,
    next_available_index: u32,
    dirty_transforms: HashSet<u32>,
    uid_to_index_map: HashMap<u32,u32>,
    index_to_use_counter: HashMap<u32,u32>,
    freed_transform_indices: Vec<u32>
}

impl TransformBuffer
{
    pub fn new(web_context: Option<&WebGl2RenderingContext>, uniform_name: &str) -> Self
    {
        let mut ubo: Option<WebGlBuffer> = None;

        match web_context
        {
            Some(context) => {
                match context.create_buffer()
                {
                    Some(buffer) => {
                        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER,Some(&buffer));
                        context.buffer_data_with_i32(WebGl2RenderingContext::UNIFORM_BUFFER, (MAX_BUFFER_SIZE * std::mem::size_of::<f32>()) as i32, WebGl2RenderingContext::STATIC_DRAW);
                        context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER,None);

                        let max_matrices = context.get_parameter(WebGl2RenderingContext::MAX_UNIFORM_BLOCK_SIZE).unwrap().as_f64().unwrap() / mem::size_of::<glm::Mat4>() as f64;
                        log(format!("Max Matrices in UBO: {}",max_matrices).as_str());

                        ubo = Some(buffer);
                    },
                    None => {}
                };
            },
            None => {}
        };

        Self {
            uniform_name: uniform_name.to_string(),
            transforms: Vec::new(),
            ubo: ubo ,
            next_available_index: 0,
            dirty_transforms: HashSet::new(),
            uid_to_index_map: HashMap::new(),
            index_to_use_counter: HashMap::new(),
            freed_transform_indices: Vec::new()
        }
    }

    pub fn bind_to_shader(&self, context: &WebGl2RenderingContext, shader: &Shader)
    {
        if self.ubo.is_none()
        {
            return;
        }

        let block_index = context.get_uniform_block_index(shader.get_shader_program(), self.uniform_name.as_str());
        context.uniform_block_binding(shader.get_shader_program(), block_index, 0);
        context.bind_buffer_base(WebGl2RenderingContext::UNIFORM_BUFFER, 0, Some(&self.ubo.as_ref().unwrap()));
    }

    pub fn unbind_from_shader(&self, context:&WebGl2RenderingContext)
    {
        //TODO: works?
        context.bind_buffer_base(WebGl2RenderingContext::UNIFORM_BUFFER, 0, None);
    }

    pub fn bind(&self, context: &WebGl2RenderingContext)
    {        
       if self.ubo.is_none()
       {
        return;
       }

       context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, Some(&self.ubo.as_ref().unwrap()));
    }

    pub fn unbind(context: &WebGl2RenderingContext)
    {
       context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, None);
    }

    pub fn set_translation(&mut self, uid: &u32, translation: &glm::Vec2)
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

    pub fn set_z(&mut self, uid: &u32, z: f32)
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

        self.transforms[*index as usize].set_z(z);

        self.dirty_transforms.insert(*index);
    }

    pub fn get_z(&self, uid: &u32) -> Option<&f32>
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

        Some(self.transforms[*index as usize].get_z())
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

    pub fn set_scale(&mut self, uid: &u32, scale: &glm::Vec2)
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

    fn increment_index_counter(&mut self, index: &u32)
    {
        let mut incremented_count = 0;

        {
            let count = self.index_to_use_counter.get(index);

            incremented_count = match count
            {
                Some(c) => {
                    c+1
                },
                None => {
                    1
                }
            };
        }

        self.index_to_use_counter.insert(*index,incremented_count);
    }

    fn decrement_index_counter(&mut self, index: &u32) -> bool
    {
        let mut decremented_count = 0;

        {
            let count = self.index_to_use_counter.get(index);

            decremented_count = match count
            {
                Some(c) => {
                    if *c > 0
                    {
                        c-1
                    } else {
                        0
                    }
                },
                None => {
                    0
                }
            };
        }
        

        self.index_to_use_counter.insert(*index,decremented_count);

        decremented_count == 0
    }

    pub fn free_transform_if_no_longer_referenced(&mut self, uid: &u32)
    {
        let index_for_uid;

        {
            index_for_uid = match self.uid_to_index_map.get(uid)
            {
                Some(i) => i.clone(),
                None => { return; } //UID doesn't have a tranform to release
            };
        }

        if self.decrement_index_counter(&index_for_uid)
        {
            //Transform can be recycled
            log(&format!("Freeing transform with index {}",index_for_uid));
            self.freed_transform_indices.push(index_for_uid);
        } 
    }

    pub fn reuse_existing_transform_index(&mut self, uid: &u32, uid_to_reuse_transform_from: &u32) -> u32 
    {
        let mut index = match self.uid_to_index_map.get(uid_to_reuse_transform_from)
        {
            Some(i) => *i,
            None => { 
                self.request_new_transform_index(uid)
            }
        };

        self.increment_index_counter(&index);
        self.uid_to_index_map.insert(uid.clone(),index.clone());

        index.clone()
    }

    pub fn request_new_transform_index(&mut self, uid: &u32) -> u32 
    {
        match self.freed_transform_indices.pop()
        {
            Some(i) => {

               if self.transforms.len() > i as usize
               {
                    log(&format!("Recycling transform with index {}",i));
                   self.transforms[i as usize].reset();
                   self.dirty_transforms.insert(i);
                   self.increment_index_counter(&i);
                   self.uid_to_index_map.insert(uid.clone(),i);
                   return i;
               }
            },

            None => {}
        };

        //UID isn't in map, make a new transform
        let mat_index = self.next_available_index;
        self.next_available_index += 1;

        self.transforms.push(Transform::new());
        self.dirty_transforms.insert(mat_index);
        self.increment_index_counter(&mat_index);
        self.uid_to_index_map.insert(uid.clone(),mat_index);

        mat_index
    }

    pub fn clear(&mut self)
    {
        //NB: doesn't clear the "next index" or the transforms. the data still lives on the buffer

        for (index,count) in &self.index_to_use_counter
        {
            //for each transform currently used, put it in the free list
            if *count > 0
            {
                self.freed_transform_indices.push(index.clone());
            }
        }

        self.dirty_transforms.clear();
        self.uid_to_index_map.clear();
        self.index_to_use_counter.clear();
    }
}

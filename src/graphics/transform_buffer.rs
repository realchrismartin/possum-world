
use crate::graphics::transform::Transform;
pub struct TransformBuffer
{
    transform_data: Vec<f32>,
    buffer_dirty: bool,
    next_available_index: u32 
}

impl TransformBuffer
{
    pub fn new() -> Self
    {
        Self {
            transform_data: Vec::<f32>::new(),
            buffer_dirty: false,
            next_available_index: 0
        }
    }

    pub fn dirty(&self) -> bool
    {
        self.buffer_dirty
    }

    pub fn set_clean(&mut self)
    {
        self.buffer_dirty = false;
    }

    //TODO: fix to use transform
    /*
    pub fn update_transform(&mut self, index: u32, transform: &glm::Mat4)
    {
        if index >= self.transform_data.len() as u32
        {
            return;
        }

        let matrix_slice = transform.as_slice();
        let mut matrix_index = 0;

        //A 4x4 matrix has 16 floats
        let offset = 16 * index;

        for i in offset..offset+16
        {
            self.transform_data[i as usize] = matrix_slice[matrix_index];
            matrix_index += 1;
        }

        self.buffer_dirty = true;
    }
    */

    pub fn request_new_transform(&mut self) -> u32 
    {
        let mat_index = self.next_available_index;
        self.next_available_index += 1;
        self.buffer_dirty = true;

        self.transform_data.extend_from_slice(glm::Mat4::identity().as_slice());

        mat_index
    }

    pub fn data(&self) -> &Vec<f32>
    {
        &self.transform_data
    }

}
use crate::graphics::transform::Transform;
pub struct TransformBuffer
{
    transforms: Vec<Transform>,
    transform_data: Vec<f32>,
    buffer_dirty: bool,
    next_available_index: u32 
}

impl TransformBuffer
{
    pub fn new() -> Self
    {
        Self {
            transforms: Vec::new(),
            transform_data: Vec::new(),
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

    pub fn set_translation(&mut self, index: u32, translation: glm::Vec3)
    {
        if self.transforms.len() <= index as usize
        {
            return;
        }        

        self.transforms[index as usize].set_translation(translation);
    }

    pub fn set_rotation(&mut self, index: u32, rotation: f32)
    {
        if self.transforms.len() <= index as usize
        {
            return;
        }       
        self.transforms[index as usize].set_rotation(rotation);
    }

    pub fn set_scale(&mut self, index: u32, scale: glm::Vec3)
    {
        if self.transforms.len() <= index as usize
        {
            return;
        }       

        self.transforms[index as usize].set_scale(scale);
    }

    //For each transform matrix, update the raw data if it needs to be updated.
    pub fn recalculate_transforms_and_update_data(&mut self)
    {
        let mut any_dirty = false;

        let mut index = 0;
        for transform in &mut self.transforms
        {
            if !transform.dirty()
            {
                index += 1;
                continue;
            }

            if index >= self.transform_data.len() as u32
            {
                return; //This should not occur.
            }

            any_dirty = true;

            let matrix = transform.calculate();

            let matrix_slice = matrix.as_slice();
            let mut matrix_index = 0;

            //Copy the data from the calculated matrix to the buffer.
            //A 4x4 matrix has 16 floats
            let offset = 16 * index;

            for i in offset..offset+16
            {
                self.transform_data[i as usize] = matrix_slice[matrix_index];
                matrix_index += 1;
            }

            transform.set_clean();

            index += 1;
        }

        if any_dirty
        {
            self.buffer_dirty = true;
        }
    }

    pub fn request_new_transform(&mut self) -> u32 
    {
        let mat_index = self.next_available_index;
        self.next_available_index += 1;
        self.buffer_dirty = true;

        self.transforms.push(Transform::new());
        self.transform_data.extend_from_slice(glm::Mat4::identity().as_slice()); //This works because a default Transform has an ID matrix.

        mat_index
    }

    pub fn data(&self) -> &Vec<f32>
    {
        &self.transform_data
    }

}
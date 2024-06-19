
#[derive(Debug)]
pub struct TransformBuffer
{
    transform_data: Vec<f32>,
    buffer_dirty: bool,
    next_available_index: usize
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

    pub fn update_matrix(&mut self, index: usize, matrix: &glm::Mat4)
    {
        if index >= self.transform_data.len()
        {
            return;
        }

        let matrix_slice = matrix.as_slice();
        let mut matrix_index = 0;

        for element in &mut self.transform_data[0..index]
        {
            //TODO: does this work?
            *element = matrix_slice[matrix_index];
            matrix_index += 1;
        }

        self.buffer_dirty = true;
    }

    pub fn add_matrix(&mut self,matrix: &glm::Mat4) -> usize
    {
        let mat_index = self.next_available_index;
        self.next_available_index += 1;
        self.buffer_dirty = true;

        self.transform_data.extend_from_slice(matrix.as_slice());

        mat_index
    }

    pub fn add_identity_matrix(&mut self) -> usize
    {
        return self.add_matrix(&glm::Mat4::identity().into());
    }

    pub fn data(&self) -> &Vec<f32>
    {
        &self.transform_data
    }

}
use nalgebra_glm::TMat4;
use crate::util::logging::log_f32;

pub struct Camera
{
    view_matrix: TMat4<f32>,
    projection_matrix: TMat4<f32>,
    dirty : bool,
    canvas_width : f32,
    canvas_height : f32
}

impl Camera
{
    pub fn new(canvas_width : f32, canvas_height : f32) -> Self
    {
        Self
        {
            view_matrix: glm::Mat4::identity(), 
            projection_matrix: glm::Mat4::identity(),
            dirty: true,
            canvas_width: canvas_width, //TODO: may change
            canvas_height: canvas_height
        }
    }

    fn update_view_matrix(&mut self)
    {
        let eye = glm::vec3(0.0,0.0,0.0);
        let target = glm::vec3(0.0,0.0,-1.0);
        let up_vector = glm::vec3(0.0,1.0,0.0);

        self.view_matrix = glm::look_at(&eye,&target,&up_vector);
    }

    fn update_projection_matrix(&mut self)
    {
        let left = -1.0; 
        let right = 1.0;
        let bottom = -1.0;
        let top = 1.0;
        let near = 1.0;
        let far = -5.0;

        self.projection_matrix = glm::ortho(left,right,bottom,top,near,far);
    }

    pub fn get_view_projection_matrix(self : &Self) -> TMat4<f32>
    {
        self.view_matrix * self.projection_matrix
    }

    pub fn dirty(&self) -> bool
    {
        return self.dirty;
    }

    pub fn recalculate(&mut self)
    {
        self.update_view_matrix();
        self.update_projection_matrix();
        self.dirty = false;
    }

}
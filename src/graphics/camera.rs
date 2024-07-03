use nalgebra_glm::TMat4;
use crate::util::logging::log;

pub struct Camera
{
    view_matrix: TMat4<f32>,
    projection_matrix: TMat4<f32>,
    dirty : bool,
    canvas_width : u32,
    canvas_height : u32
}

impl Camera
{
    pub fn new(canvas_width : u32, canvas_height : u32) -> Self
    {
        Self
        {
            view_matrix: glm::Mat4::identity(), 
            projection_matrix: glm::Mat4::identity(),
            dirty: true,
            canvas_width,
            canvas_height
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
        let bottom = -1.0;
        let top = 1.0;
        let near = 0.1;
        let far = 20.0;

        let aspect_ratio = self.canvas_width as f32 / self.canvas_height as f32;
        let left = -aspect_ratio;
        let right = aspect_ratio;

        log(format!("New aspect ratio: {} with w: {} and h: {}",aspect_ratio,self.canvas_width,self.canvas_height).as_str());

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

    pub fn get_canvas_width(&self) -> u32
    {
        self.canvas_width
    }

    pub fn get_canvas_height(&self) -> u32
    {
        self.canvas_height
    }

    pub fn set_canvas_dimensions(&mut self, x: u32, y: u32)
    {
        log(format!("Canvas size set to: {} {}",x,y).as_str());

        self.canvas_width = x;
        self.canvas_height = y;
        self.dirty = true;
    }

}
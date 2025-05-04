use nalgebra_glm::TMat4;
use crate::util::util::world_position_to_screen_translation;

pub struct Camera
{
    view_matrix: TMat4<f32>,
    projection_matrix: TMat4<f32>,
    dirty : bool,
    canvas_width : u32,
    canvas_height : u32,
    eye: glm::Vec3,
    target: glm::Vec3
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
            canvas_height,
            eye: glm::vec3(0.0,0.0,1.0),
            target: glm::vec3(0.0,0.0,0.0)
        }
    }

    fn update_view_matrix(&mut self)
    {
        let up_vector = glm::vec3(0.0,1.0,0.0);
        self.view_matrix = glm::look_at(&self.eye,&self.target,&up_vector);
    }

    fn update_projection_matrix(&mut self)
    {
        let near = -10000.0;
        let far = 10000.0;

        //TODO: AR is unused.
        //let aspect_ratio = self.canvas_width as f32 / self.canvas_height as f32;

        let left = -1.0;
        let right = 1.0;
        let bottom = -1.0;
        let top = 1.0;

        self.projection_matrix = glm::ortho(left,right,bottom,top,near,far);
    }

    pub fn get_view_projection_matrix(self : &Self) -> TMat4<f32>
    {
        self.projection_matrix * self.view_matrix
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
        self.canvas_width = x;
        self.canvas_height = y;
        self.dirty = true;
    }

    pub fn set_camera_world_position(&mut self, position: &glm::Vec2)
    {
        let screen_translation = world_position_to_screen_translation(&position,&glm::vec2(self.canvas_width as f32,self.canvas_height as f32));

        self.eye.x = screen_translation.x;
        self.eye.y = screen_translation.y;

        self.target.x = screen_translation.x;
        self.target.y = screen_translation.y;
        self.dirty = true;
    }
}
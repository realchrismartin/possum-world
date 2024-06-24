use crate::util::logging::log;

//Given a top left corner, the dimensions of a rectangle, and the dimensions of a texture, get a set of 4 texture coordinates for the rectangle's extents
pub fn get_rectangular_texture_coordinates(top_left_pixel_coordinate: &[i32;2], dimensions: &[i32;2], tex_dimensions: &[i32;2]) -> [[f32;2];4]
{
    let x = top_left_pixel_coordinate[0] as f32 / tex_dimensions[0] as f32;
    let y = top_left_pixel_coordinate[1] as f32 / tex_dimensions[1] as f32;

    let width = dimensions[0] as f32 / tex_dimensions[0] as f32;
    let height = dimensions[1] as f32 / tex_dimensions[1] as f32;

    let left_top = [x,y];
    let left_bottom = [x,y + height];
    let right_bottom = [x + width,y + height];
    let right_top = [x + width,y];

    return [left_top,left_bottom,right_bottom,right_top];
}

pub fn world_position_to_screen_translation(position: &glm::Vec2, world_size: &glm::Vec2) -> glm::Vec3
{
    //Screen is -1.0 to 1.0 (NDCs). Width and height of the screen are de facto each 2.0
    //This is how much each position unit is worth in screen space:
    let x_factor = 2.0 / world_size.x;
    let y_factor = 2.0 / world_size.y;

    //Calculate how much screen space the position calls for
    let x_pre = position.x * x_factor;
    let y_pre = position.y * y_factor;

    //Offset the values because we start from -1 and not 0
    let x = x_pre - 1.0;
    let y = y_pre - 1.0;

    glm::vec3(x,y,0.0)
}
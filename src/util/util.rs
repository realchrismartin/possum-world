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
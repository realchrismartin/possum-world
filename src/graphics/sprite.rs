use web_sys::WebGl2RenderingContext;
use crate::graphics::renderable::Renderable;

pub struct Sprite
{
    vertices: [f32;28],
    indices: [u32;6]
}

impl Sprite
{
    pub fn new(texture_dimensions: [i32;2], texture_coordinates: [i32;2], size: [i32;2], texture_index: u32, transform_index: u32, z: f32) -> Self 
    {
        let tex_coords = Self::get_texture_coordinates(texture_coordinates,size,texture_dimensions);

        Sprite 
        {
            vertices: 
            [
                -1.0,1.0,z,
                transform_index as f32,
                tex_coords[0][0], tex_coords[0][1],
                texture_index as f32,

                -1.0,-1.0,z,
                transform_index as f32,
                tex_coords[1][0], tex_coords[1][1],
                texture_index as f32,

                1.0,-1.0,z,
                transform_index as f32,
                tex_coords[2][0], tex_coords[2][1],
                texture_index as f32,

                1.0,1.0,z,
                transform_index as f32,
                tex_coords[3][0], tex_coords[3][1],
                texture_index as f32,
            ],
            indices: [0,1,2,2,3,0]
        }
    }

    fn get_texture_coordinates(top_left_pixel_coordinate: [i32;2], dimensions: [i32;2], texture_dimensions: [i32;2]) -> [[f32;2];4]
    {
        let x = top_left_pixel_coordinate[0] as f32 / texture_dimensions[0] as f32;
        let y = top_left_pixel_coordinate[1] as f32 / texture_dimensions[1] as f32;

        let width = dimensions[0] as f32 / texture_dimensions[0] as f32;
        let height = dimensions[1] as f32 / texture_dimensions[1] as f32;

        let left_top = [x,y];
        let left_bottom = [x,y + height];
        let right_bottom = [x + width,y + height];
        let right_top = [x + width,y];

        return [left_top,left_bottom,right_bottom,right_top];
    }
}


impl Renderable for Sprite
{
    fn init_vertex_layout(context: &WebGl2RenderingContext)
    {
        let position_attribute_location = 0;
        let model_matrix_index_attribute_location = 1;
        let texture_coordinates_attribute_location = 2;
        let texture_index_attribute_location= 3;
        
        let float_size = std::mem::size_of::<f32>() as i32;

        //Enable each vertex attribute
        //When this happens, whatever bound VBO there is becomes associated with this VAO.
        //We assume the VAO/VBO is already bound
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            Self::get_stride() as i32 * float_size,
            0
        );

        context.vertex_attrib_pointer_with_i32(
            model_matrix_index_attribute_location as u32,
            1,
            WebGl2RenderingContext::FLOAT,
            false,
            Self::get_stride() as i32 * float_size,
            3 * float_size
        );

        context.vertex_attrib_pointer_with_i32(
            texture_coordinates_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            Self::get_stride() as i32 * float_size,
            4 * float_size
        );

        context.vertex_attrib_pointer_with_i32(
            texture_index_attribute_location as u32,
            1,
            WebGl2RenderingContext::FLOAT,
            false,
            Self::get_stride() as i32 * float_size,
            6 * float_size
        );

        context.enable_vertex_attrib_array(position_attribute_location as u32);
        context.enable_vertex_attrib_array(model_matrix_index_attribute_location as u32);
        context.enable_vertex_attrib_array(texture_coordinates_attribute_location as u32);
        context.enable_vertex_attrib_array(texture_index_attribute_location as u32);
    }

    fn get_vertices(&self) -> &[f32]
    {
        return &self.vertices;
    }

    fn get_indices(&self) -> &[u32]
    {
        return &self.indices;
    }

    fn get_stride() -> usize
    {
        return 7;
    }

}
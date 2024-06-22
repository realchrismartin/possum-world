use web_sys::WebGl2RenderingContext;
use crate::graphics::vertex_layout::VertexLayout;

pub trait Renderable
{
    fn init_vertex_layout(context: &WebGl2RenderingContext)
    {

        //TODO: fix
        //TODO: infer the stride 

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

    fn get_vertices(&self) -> &[f32];
    fn get_indices(&self) -> &[u32];
    fn should_be_drawn(&self) -> bool;
    fn set_should_be_drawn(&mut self, state: bool);
    fn get_vertex_layout() -> VertexLayout;
}
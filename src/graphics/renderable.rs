use web_sys::WebGl2RenderingContext;
use crate::graphics::vertex_layout::VertexLayout;

pub trait Renderable
{
    fn init_vertex_layout(context: &WebGl2RenderingContext)
    {
        let vertex_layout = Self::get_vertex_layout();

        let float_size = std::mem::size_of::<f32>() as i32;

        let stride : i32 = float_size * Self::get_stride();

        let mut offset : i32 = 0;

        for vertex_layout_element in vertex_layout.get_elements()
        {
            context.vertex_attrib_pointer_with_i32(
                vertex_layout_element.location,
                vertex_layout_element.size,
                WebGl2RenderingContext::FLOAT,
                false,
                stride,
                offset 
            );

            context.enable_vertex_attrib_array(vertex_layout_element.location);

            offset += vertex_layout_element.size * float_size;
        }
    }

    fn get_stride() -> i32
    {
        //TODO: this is an overly expensive vector-creation to produce this value.
        Self::get_vertex_layout().get_elements().iter().map(|element| element.size ).sum::<i32>()
    }

    fn get_vertices(&self) -> &[f32];
    fn get_indices(&self) -> &[u32];
    fn should_be_drawn(&self) -> bool;
    fn set_should_be_drawn(&mut self, state: bool);
    fn get_vertex_layout() -> VertexLayout;
}
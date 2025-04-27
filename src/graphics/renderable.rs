use web_sys::WebGl2RenderingContext;
use crate::graphics::vertex_layout::VertexLayout;
use crate::RenderState;

//A Renderable is:
// Some static metadata on how to set up a Vertex Buffer a certain way
// Using this data, the renderer can set up a buffer for a renderable type and hold the data, passing back a lightweight handle that knows where the data is.
pub trait Renderable
{
    fn init_vertex_layout(context: &WebGl2RenderingContext) where Self: Sized
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

    fn get_stride() -> i32 where Self: Sized
    {
        //TODO: this is an overly expensive vector-creation to produce this value.
        Self::get_vertex_layout().get_elements().iter().map(|element| element.size ).sum::<i32>()
    }

    //Given the current render state and a renderable config to get vertices for, statically generate vertices for the renderable
    fn get_vertices(&self, render_state: &RenderState, model_matrix_transform_index: u32) -> Vec<f32> where Self: Sized;

    //This has to always be the same for any given Renderable type (ie consistent for that type) TODO - confirm if still true?
    fn get_indices(&self) -> Vec<u32> where Self: Sized;

    //Generate the vertex layout for this renderable
    fn get_vertex_layout() -> VertexLayout where Self: Sized;

    fn get_draw_type() -> u32 where Self: Sized;

    //TODO: remove this;
    fn get_size(&self) -> &[i32;2];
}
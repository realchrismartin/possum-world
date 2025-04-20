use web_sys::WebGl2RenderingContext;
use crate::graphics::vertex_layout::VertexLayout;

//This struct may contain more data than is needed for any given Renderable. TODO: address?
#[derive(Clone)]
pub struct RenderableConfig
{
    texture_coordinates: [i32;2],
    texture_dimensions: [i32;2], //TODO: storing this repeatedly/unnecessary
    size: [i32;2],
    world_size_ratio: [f32;2],
    texture_index: u32
}

impl RenderableConfig
{
    pub fn new(tex_coordinates: [i32;2], sprite_size: [i32;2], tex_index: u32) -> Self 
    {
        Self
        {
            texture_coordinates :tex_coordinates,
            texture_dimensions: [1,1], //Updated by render_state later
            size: sprite_size,
            world_size_ratio: [1.0,1.0], //Updated by render_state later
            texture_index: tex_index
        }
    }

    pub fn get_texture_index(&self) -> u32
    {
        self.texture_index
    }

    pub fn get_texture_coordinates(&self) -> &[i32;2]
    {
        &&self.texture_coordinates
    }

    pub fn get_size(&self) -> &[i32;2]
    {
        &&self.size
    }

    pub fn get_texture_dimensions(&self) -> &[i32;2]
    {
        &&self.texture_dimensions
    }

    pub fn get_world_size_ratio(&self) -> &[f32;2]
    {
        &&self.world_size_ratio
    }

    pub fn set_texture_dimensions(&mut self, dimensions: &[i32;2])
    {
        self.texture_dimensions = *dimensions;
    }

    pub fn set_world_size_ratio(&mut self, current_world_size: &[f32;2])
    {
        self.world_size_ratio = [self.size[0] as f32 / current_world_size[0],self.size[1] as f32 / current_world_size[1]];
    }
}

//A Renderable is:
//a) Some static metadata on how to set up a Vertex Buffer a certain way
//b) When derived, an object that is a handle containing indices that point to specific spots on EBO / transform buffers.
// Using this data, the renderer can set up a buffer for a renderable type and hold the data, passing back a lightweight handle that knows where the data is.
pub trait Renderable
{
    fn new(uid: u32, transform_location: u32, size: [i32;2]) -> Self where Self: Sized;

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

    //Given a renderable config, generate vertices for this renderable
    fn get_vertices(&self, renderable_config: &RenderableConfig) -> Vec<f32>;

    //Given a renderable config, generate indices for this renderable
    //This has to always be the same for any given Renderable type (ie consistent for that type)
    fn get_indices() -> Vec<u32> where Self: Sized;

    //Generate the vertex layout for this renderable
    fn get_vertex_layout() -> VertexLayout where Self: Sized;

    //Get the location of this renderable's transform on its buffer
    fn get_transform_location(&self) -> u32;

    fn get_draw_type() -> u32 where Self: Sized;

    fn get_size(&self) -> &[i32;2];

    fn get_uid(&self) -> &u32;
}
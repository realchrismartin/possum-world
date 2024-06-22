use crate::graphics::renderable::{Renderable, RenderableConfig};
use crate::graphics::vertex_layout::{VertexLayout,VertexLayoutElement};
use crate::util::util::get_rectangular_texture_coordinates;
use std::ops::Range;

//Like all Renderables, a Sprite is a handle that points to locations on our buffers.
//It doesn't hold vertex or index data. That data is generated once on upload to the GPU.
pub struct Sprite
{
    element_location: Range<i32>,
    transform_location: u32,
}

impl Renderable for Sprite
{
    fn new(element_location: &Range<i32>, transform_location: u32) -> Self 
    {
        Self 
        {
            element_location: *element_location,
            transform_location,
        }
    }

    fn get_vertex_layout() -> super::vertex_layout::VertexLayout
    {
       VertexLayout::new(vec![
            VertexLayoutElement { location: 0, size: 3}, //Position
            VertexLayoutElement { location: 1, size: 1}, //Model matrix index
            VertexLayoutElement { location: 2, size: 2}, //Texture coords
            VertexLayoutElement { location: 3, size: 1}, //Texture index
       ])
    }

    fn get_vertices(&self, renderable_config: &RenderableConfig) -> Vec<f32>
    {
        //TODO: maybe later don't use self at all here

        let tex_coords = get_rectangular_texture_coordinates(renderable_config.get_texture_coordinates(), 
            renderable_config.get_size(), renderable_config.get_texture_dimensions());

        vec![
            -1.0,1.0,
            renderable_config.get_z(),
            self.transform_location as f32,
            tex_coords[0][0], tex_coords[0][1],
            renderable_config.get_texture_index() as f32,

            -1.0,-1.0,
            renderable_config.get_z(),
            self.transform_location as f32,
            tex_coords[1][0], tex_coords[1][1],
            renderable_config.get_texture_index() as f32,

            1.0,-1.0,
            renderable_config.get_z(),
            self.transform_location as f32,
            tex_coords[2][0], tex_coords[2][1],
            renderable_config.get_texture_index() as f32,

            1.0,1.0,
            renderable_config.get_z(),
            self.transform_location as f32,
            tex_coords[3][0], tex_coords[3][1],
            renderable_config.get_texture_index() as f32,
        ]
    }

    fn get_indices(&self, renderable_config: &RenderableConfig) -> Vec<u32>
    {
        vec![0,1,2,2,3,0]
    }

    fn get_element_location(&self) -> &Range<i32> 
    {
      &self.element_location  
    }

    fn get_transform_location(&self) -> u32 
    {
       self.transform_location 
    }
}
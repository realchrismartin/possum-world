use crate::graphics::renderable::Renderable;
use crate::graphics::vertex_layout::{VertexLayout,VertexLayoutElement};

pub struct SpriteConfig
{
    texture_coordinates: [i32;2],
    size: [i32;2],
    texture_index: u32,
    z: f32
}

impl SpriteConfig
{
    pub fn new(tex_coordinates: [i32;2], sprite_size: [i32;2], tex_index: u32, z_ind: f32) -> Self 
    {
        Self
        {
            texture_coordinates :tex_coordinates,
            size: sprite_size,
            texture_index: tex_index,
            z: z_ind
        }
    }

    pub fn get_texture_index(&self) -> u32
    {
        self.texture_index
    }
}

pub struct Sprite
{
    vertices: [f32;28],
    indices: [u32;6],
    transform_index: u32,
    should_be_drawn: bool 
}

impl Sprite
{
    pub fn new(sprite_config : &SpriteConfig, transform_index: u32, texture_dimensions: [u32;2]) -> Self 
    {
        let tex_coords = Self::get_texture_coordinates(sprite_config.texture_coordinates,sprite_config.size,texture_dimensions);

        Sprite 
        {
            vertices: 
            [
                -1.0,1.0,sprite_config.z,
                transform_index as f32,
                tex_coords[0][0], tex_coords[0][1],
                sprite_config.texture_index as f32,

                -1.0,-1.0,sprite_config.z,
                transform_index as f32,
                tex_coords[1][0], tex_coords[1][1],
                sprite_config.texture_index as f32,

                1.0,-1.0,sprite_config.z,
                transform_index as f32,
                tex_coords[2][0], tex_coords[2][1],
                sprite_config.texture_index as f32,

                1.0,1.0,sprite_config.z,
                transform_index as f32,
                tex_coords[3][0], tex_coords[3][1],
                sprite_config.texture_index as f32,
            ],
            indices: [0,1,2,2,3,0],
            should_be_drawn: true,
            transform_index: transform_index
        }
    }

    pub fn get_transform_index(&self) -> u32
    {
        self.transform_index
    }

    fn get_texture_coordinates(top_left_pixel_coordinate: [i32;2], dimensions: [i32;2], texture_dimensions: [u32;2]) -> [[f32;2];4]
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
    fn get_vertex_layout() -> super::vertex_layout::VertexLayout
    {
       VertexLayout::new(vec![
            VertexLayoutElement { location: 0, size: 3}, //Position
            VertexLayoutElement { location: 1, size: 1}, //Model matrix index
            VertexLayoutElement { location: 2, size: 2}, //Texture coords
            VertexLayoutElement { location: 3, size: 1}, //Texture index
       ])
    }

    fn get_vertices(&self) -> &[f32]
    {
        return &self.vertices;
    }

    fn get_indices(&self) -> &[u32]
    {
        return &self.indices;
    }

    fn should_be_drawn(&self) -> bool
    {
        return self.should_be_drawn;
    }

    fn set_should_be_drawn(&mut self, state: bool)
    {
        self.should_be_drawn = state;
    }

}
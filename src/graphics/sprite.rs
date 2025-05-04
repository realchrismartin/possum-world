use web_sys::WebGl2RenderingContext;

use crate::graphics::renderable::Renderable;
use crate::graphics::vertex_layout::{VertexLayout,VertexLayoutElement};
use crate::util::util::get_rectangular_texture_coordinates;
use crate::RenderState;
use crate::component::component::Component;

#[derive(Clone)]
pub struct Sprite {
    renderable_uid: u32,
    texture_coordinates: [i32;2],
    size: [i32;2],
    texture_index: u32,
    starting_world_position: glm::Vec2,
    starting_z: f32,
    starting_scale: glm::Vec2
}

impl Sprite
{
    pub fn new(tex_coordinates: [i32;2], sprite_size: [i32;2], tex_index: u32) -> Self 
    {
        Self
        {
            renderable_uid: 0,  //TODO: better default
            texture_coordinates :tex_coordinates,
            size: sprite_size,
            texture_index: tex_index,
            starting_world_position: glm::vec2(0.0,0.0),
            starting_scale: glm::vec2(1.0,1.0),
            starting_z: 0.0
        }
    }

    pub fn new_with_position(tex_coordinates: [i32;2], sprite_size: [i32;2], tex_index: u32, starting_world_position: glm::Vec2, starting_z: f32, starting_scale: glm::Vec2) -> Self 
    {
        Self
        {
            renderable_uid: 0,  //TODO: better default
            texture_coordinates :tex_coordinates,
            size: sprite_size,
            texture_index: tex_index,
            starting_world_position: starting_world_position,
            starting_scale: starting_scale,
            starting_z: starting_z
        }
    }
}

impl Component for Sprite
{
}

impl Renderable for Sprite
{
    fn get_vertex_layout() -> super::vertex_layout::VertexLayout
    {
       VertexLayout::new(vec![
            VertexLayoutElement { location: 0, size: 3}, //Position
            VertexLayoutElement { location: 1, size: 1}, //Model matrix transform index
            VertexLayoutElement { location: 2, size: 2}, //Texture coords
            VertexLayoutElement { location: 3, size: 1}, //Texture index
       ])
    }

    fn get_vertices(&self, render_state: &RenderState, model_matrix_transform_index: u32) -> Vec<f32>
    {
        let texture_dimensions = match render_state.get_texture(self.texture_index)
        {
            Some(t) => t.get_dimensions(),
            None => { [1,1] }
        };

        let tex_coords = get_rectangular_texture_coordinates(&self.texture_coordinates,&self.size,&texture_dimensions);

        //Local size is set according to how big the sprite should be in comparison to the canvas size.
        //TODO: sprites may be 2x too big, but "fixing" this breaks position
        let x_axis = self.size[0] as f32 / render_state.get_canvas_size_x() as f32;
        let y_axis = self.size[1] as f32 / render_state.get_canvas_size_y() as f32;

        vec![
            -x_axis,y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[0][0], tex_coords[0][1],
            self.texture_index as f32,

            -x_axis,-y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[1][0], tex_coords[1][1],
            self.texture_index as f32,

            x_axis,-y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[2][0], tex_coords[2][1],
            self.texture_index as f32,

            x_axis,y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[3][0], tex_coords[3][1],
            self.texture_index as f32,
        ]
    }

    fn get_indices(&self) -> Vec<u32>
    {
        vec![0,1,2,2,3,0]
    }

    fn get_draw_type() -> u32
    {
        WebGl2RenderingContext::TRIANGLES
    }

    fn get_size(&self) -> &[i32;2]
    {
        &&self.size
    }

    fn get_starting_world_position(&self) -> Option<&glm::Vec2> 
    {
        Some(&&self.starting_world_position)
    }

    fn get_starting_scale(&self) -> Option<&glm::Vec2> 
    {
        Some(&&self.starting_scale)
    }

    fn get_starting_z(&self) -> Option<f32> 
    {
        Some(self.starting_z)
    }

    fn get_renderable_uid(&self) -> u32
    {
        self.renderable_uid
    }

    fn set_renderable_uid(&mut self, uid: u32)
    {
        self.renderable_uid = uid;
    }
}
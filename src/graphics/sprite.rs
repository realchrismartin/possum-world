use web_sys::WebGl2RenderingContext;

use crate::graphics::renderable::Renderable;
use crate::graphics::vertex_layout::{VertexLayout,VertexLayoutElement};
use crate::util::util::get_rectangular_texture_coordinates;
use crate::RenderState;

#[derive(Clone)]
pub struct Sprite {
    texture_coordinates: [i32;2],
    size: [i32;2],
    texture_index: u32
}

impl Sprite
{
    pub fn new(tex_coordinates: [i32;2], sprite_size: [i32;2], tex_index: u32) -> Self 
    {
        Self
        {
            texture_coordinates :tex_coordinates,
            size: sprite_size,
            texture_index: tex_index
        }
    }
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
}
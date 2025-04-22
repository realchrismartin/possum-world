use web_sys::WebGl2RenderingContext;

use crate::graphics::renderable::{Renderable, RenderableConfig};
use crate::graphics::vertex_layout::{VertexLayout,VertexLayoutElement};
use crate::util::util::get_rectangular_texture_coordinates;
use crate::RenderState;

//Like all Renderables, a Sprite is a handle that points to locations on our buffers.
//It doesn't hold vertex or index data. That data is generated once on upload to the GPU.
#[derive(Clone)]
pub struct Sprite {}

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

    fn get_vertices(render_state: &RenderState, renderable_config: &RenderableConfig, model_matrix_transform_index: u32) -> Vec<f32>
    {
        let texture_dimensions = match render_state.get_texture(renderable_config.get_texture_index())
        {
            Some(t) => t.get_dimensions(),
            None => { [1,1] }
        };

        let tex_coords = get_rectangular_texture_coordinates(renderable_config.get_texture_coordinates(), 
            renderable_config.get_size(), &texture_dimensions);

        //Local size is set according to how big the sprite should be in comparison to the canvas size.
        let x_axis = renderable_config.get_size()[0] as f32 / render_state.get_canvas_size_x() as f32;
        let y_axis = renderable_config.get_size()[1] as f32 / render_state.get_canvas_size_y() as f32;

        vec![
            -x_axis,y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[0][0], tex_coords[0][1],
            renderable_config.get_texture_index() as f32,

            -x_axis,-y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[1][0], tex_coords[1][1],
            renderable_config.get_texture_index() as f32,

            x_axis,-y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[2][0], tex_coords[2][1],
            renderable_config.get_texture_index() as f32,

            x_axis,y_axis,0.0,
            model_matrix_transform_index as f32,
            tex_coords[3][0], tex_coords[3][1],
            renderable_config.get_texture_index() as f32,
        ]
    }

    fn get_indices() -> Vec<u32>
    {
        vec![0,1,2,2,3,0]
    }

    fn get_draw_type() -> u32
    {
        WebGl2RenderingContext::TRIANGLES
    }
}
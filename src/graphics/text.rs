use web_sys::WebGl2RenderingContext;

use crate::graphics::renderable::Renderable;
use crate::graphics::vertex_layout::{VertexLayout,VertexLayoutElement};
use crate::util::util::{get_character_size, get_character_texture_coordinates};

use crate::util::logging::log;

use crate::RenderState;

#[derive(Clone)]
pub struct Text {
    content: String,
    size: [i32;2],
    texture_index: u32,
    pixel_space_between_letters: u32
}

impl Text 
{
    pub fn new(content: &str) -> Self 
    {
        Self
        {
            content: String::from(content),
            size: [1,1], //TODO
            texture_index: 1, //TODO: hardcoded
            pixel_space_between_letters: 4
        }
    }
}

impl Renderable for Text
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
        //NB: texture index is hardcoded for font
        let texture_dimensions = match render_state.get_texture(self.texture_index)
        {
            Some(t) => t.get_dimensions(),
            None => { [1,1] }
        };

        let mut vertices = Vec::<f32>::new();


        let mut character_size_iterator = self.content.chars();

        let x_pixel_padding = (self.pixel_space_between_letters as f32 / render_state.get_canvas_size_x() as f32);

        let mut total_x_canvas_size : f32 = 0.0;
        let mut tallest_character_height : f32 = 0.0;

        while let Some(character) = character_size_iterator.next()
        {
            let character_size = get_character_size(&character);

            //Local size is set according to how big the character should be in comparison to the canvas size.
            let x_axis = character_size[0] as f32 / render_state.get_canvas_size_x() as f32;
            let y_axis = character_size[1] as f32 / render_state.get_canvas_size_y() as f32;

            total_x_canvas_size += ((x_axis + x_pixel_padding) * 2.0);
            tallest_character_height = tallest_character_height.max(y_axis);
        }

        //Establish offsets so that the center of the combined text is at 0,0
        //TODO: Later, we may wish to add an option that doesn't center the text.
        let x_canvas_offset = total_x_canvas_size / 2.0;

        let mut character_iterator = self.content.chars();
        let mut x_used : f32 = 0.0;

        while let Some(character) = character_iterator.next()
        {
            let character_size = get_character_size(&character);

            //Local size is set according to how big the character should be in comparison to the canvas size.
            let x_axis = character_size[0] as f32 / render_state.get_canvas_size_x() as f32;
            let y_axis = character_size[1] as f32 / render_state.get_canvas_size_y() as f32;

            match character
            {
                ' ' => {
                    x_used += (x_axis * 2.0) + x_pixel_padding;
                    continue;
                }
                _ => {}
            };

            //Set to the amount of space it takes to make all the characters flush with the bottom line
            let y_canvas_offset = tallest_character_height - y_axis;

            let tex_coords = get_character_texture_coordinates(&character, &texture_dimensions);

            let char_vertex_vec = vec![
                -x_axis + x_used - x_canvas_offset,y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[0][0], tex_coords[0][1],
                self.texture_index as f32,

                -x_axis + x_used - x_canvas_offset,-y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[1][0], tex_coords[1][1],
                self.texture_index as f32,

                x_axis + x_used - x_canvas_offset,-y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[2][0], tex_coords[2][1],
                self.texture_index as f32,

                x_axis + x_used - x_canvas_offset,y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[3][0], tex_coords[3][1],
                self.texture_index as f32,
            ];

            vertices.extend(&char_vertex_vec);
            x_used += (x_axis * 2.0) + x_pixel_padding;
        }

        vertices
    }

    fn get_indices(&self) -> Vec<u32>
    {
        let mut indices = Vec::<u32>::new();

        const indices_per_char : u32 = 4;

        let chars_so_far : u32 = 0;

        for index in 0..self.content.chars().count()
        {
            let i : u32 = index as u32;

            indices.push(0 + (i * indices_per_char));
            indices.push(1 + (i * indices_per_char));
            indices.push(2 + (i * indices_per_char));
            indices.push(2 + (i * indices_per_char));
            indices.push(3 + (i * indices_per_char));
            indices.push(0 + (i * indices_per_char));
        }

        indices
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
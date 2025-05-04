use web_sys::WebGl2RenderingContext;

use crate::graphics::renderable::Renderable;
use crate::graphics::vertex_layout::{VertexLayout,VertexLayoutElement};
use crate::graphics::font::Font;
use crate::util::util::get_rectangular_texture_coordinates;
use crate::component::component::Component;

use crate::RenderState;

const INDICES_PER_CHAR : u32 = 4;

#[derive(Clone)]
pub struct Text {
    renderable_uid: u32,
    content: String,
    font: Font,
    size: [i32;2],
    starting_world_position: glm::Vec3
}

impl Component for Text
{
}

impl Text 
{
    pub fn new(content: &str, font: &Font) -> Self 
    {
        Self
        {
            renderable_uid: 0, //TODO: better deefault
            content: String::from(content),
            font: font.clone(),
            size: [1,1], //TODO: unused
            starting_world_position: glm::vec3(0.0,0.0,0.0),
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
        let texture_index = match self.font.get_texture_index()
        {
            Some(i) => i,
            None => 2 //2 is the default tex index
        };

        let kerning_width = match self.font.get_kerning_pixel_width()
        {
            Some(v) => v as f32 / render_state.get_canvas_size_x() as f32,
            None => { 0.0 }
        };

        let whitespace_width = match self.font.get_whitespace_pixel_width()
        {
            Some(v) => v as f32 / render_state.get_canvas_size_x() as f32,
            None => { 0.0 }
        };

        let texture_dimensions = match render_state.get_texture(texture_index)
        {
            Some(t) => t.get_dimensions(),
            None => { [1,1] }
        };

        let mut character_size_iterator = self.content.chars();

        let mut total_x_canvas_size : f32 = 0.0;
        let mut tallest_character_height : f32 = 0.0;

        while let Some(character) = character_size_iterator.next()
        {
            match character
            {
                ' ' => {
                    total_x_canvas_size += (whitespace_width * 2.0) + kerning_width;
                    continue;
                }
                _ => {}
            };

            let char_data = match self.font.get_character_data(&character)
            {
                Some(f) => f,
                None => { continue; }
            };

            //Local size is set according to how big the character should be in comparison to the canvas size.
            let character_size = char_data.get_size();
            let x_axis = character_size[0] as f32 / render_state.get_canvas_size_x() as f32;
            let y_axis = character_size[1] as f32 / render_state.get_canvas_size_y() as f32;

            total_x_canvas_size += x_axis + kerning_width;
            tallest_character_height = tallest_character_height.max(y_axis);
        }

        let mut vertices = Vec::<f32>::new();

        //Establish offsets so that the center of the combined text is at 0,0
        //TODO: Later, we may wish to add an option that doesn't center the text.
        let x_canvas_offset = total_x_canvas_size / 2.0;

        let mut character_iterator = self.content.chars();
        let mut x_used : f32 = 0.0;

        while let Some(character) = character_iterator.next()
        {
            //For whitespace, just pad the next character
            match character
            {
                ' ' => {
                    x_used += (whitespace_width * 2.0) + kerning_width;
                    continue;
                }
                _ => {}
            };

            let char_data = match self.font.get_character_data(&character)
            {
                Some(f) => f,
                None => { continue; }
            };

            let character_size = char_data.get_size();
            let character_tex_coords = char_data.get_tex_coords();

            //Local size is set according to how big the character should be in comparison to the canvas size.
            let x_axis = character_size[0] as f32 / render_state.get_canvas_size_x() as f32;
            let y_axis = character_size[1] as f32 / render_state.get_canvas_size_y() as f32;

            //TODO: position might be slightly off on both axes, for different reasons here

            //Set to the amount of space it takes to make all the characters flush with the bottom line
            let y_canvas_offset = tallest_character_height - y_axis;

            let tex_coords = get_rectangular_texture_coordinates(character_tex_coords, character_size, &texture_dimensions);

            //TODO: slight bug here if characters differ in size(?)
            //e.g. M overlaps with whatever is before it because it's larger
            //Alternately, position might be slightly off since local doesn't overlap zero, unless adjustment is working right
            let char_vertex_vec = vec![
                x_used - x_canvas_offset,y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[0][0], tex_coords[0][1],
                texture_index as f32,

                x_used - x_canvas_offset,-y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[1][0], tex_coords[1][1],
                texture_index as f32,

                x_axis + x_used - x_canvas_offset,-y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[2][0], tex_coords[2][1],
                texture_index as f32,

                x_axis + x_used - x_canvas_offset,y_axis - y_canvas_offset,0.0,
                model_matrix_transform_index as f32,
                tex_coords[3][0], tex_coords[3][1],
                texture_index as f32,
            ];

            vertices.extend(&char_vertex_vec);
            x_used += x_axis + kerning_width;
        }

        vertices
    }

    fn get_indices(&self) -> Vec<u32>
    {
        let mut indices = Vec::<u32>::new();

        for index in 0..self.content.chars().count()
        {
            let i : u32 = index as u32;

            indices.push(0 + (i * INDICES_PER_CHAR));
            indices.push(1 + (i * INDICES_PER_CHAR));
            indices.push(2 + (i * INDICES_PER_CHAR));
            indices.push(2 + (i * INDICES_PER_CHAR));
            indices.push(3 + (i * INDICES_PER_CHAR));
            indices.push(0 + (i * INDICES_PER_CHAR));
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

    fn get_starting_world_position(&self) -> Option<&glm::Vec3> 
    {
        Some(&&self.starting_world_position)
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
use wasm_bindgen::prelude::*;
/*
use std::panic;
extern crate console_error_panic_hook;
panic::set_hook(Box::new(console_error_panic_hook::hook));
 */

extern crate nalgebra_glm as glm;

mod state;
mod util;
mod graphics;
mod game;

use web_sys::{Document, HtmlImageElement};

use state::game_state::GameState;
use state::input_state::InputState;
use state::render_state::RenderState;
use game::entity::Entity;
use core::ops::Range;

use graphics::sprite::Sprite;
use crate::util::logging::log_f32;

#[wasm_bindgen]
pub struct Game
{
    game_state: GameState,
    render_state: Option<RenderState>,
    input_state: InputState,
    entities: Vec<Entity> //TODO
}

#[wasm_bindgen]
impl Game
{
    pub fn new() -> Self
    {
        Self
        {
            game_state: GameState::new(),
            render_state: None::<RenderState>,
            input_state: InputState::new(),
            entities: Vec::new()
        }
    }

    pub fn init_renderer(&mut self, document: &Document)
    {
        self.render_state = RenderState::new(document);

    }
    
    pub fn load_shader(&mut self, vert_shader: &str, frag_shader: &str)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.set_shader(vert_shader, frag_shader);
    }

    pub fn load_texture(&mut self, img: HtmlImageElement)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.load_texture(img);
    }

    pub fn init_render_data(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.submit_camera_uniforms(); //TODO: if we change perspective, do this more than once.

        //Possum!
        //TODO: later, move transform data somewhere else.
        //NB: Z scale has to be 0 or we get clipped right now.
        let mut s_w: glm::Mat4 = glm::Mat4::identity().into();
        let mut s_2_w: glm::Mat4 = glm::Mat4::identity().into();

        let scale = glm::vec3(0.1,0.1,0.0);
        s_w = glm::scale(&s_w, &scale);

        let scale_2 = glm::vec3(2.0,2.0,0.0);
        s_2_w = glm::scale(&s_w, &scale_2);

        //TODO: encapsulate the transform buffer better later
        let transform_1 = render_state.transform_buffer().add_matrix(&s_w);
        let sprite = Sprite::new([500,500],[0,0],[38,17],0,transform_1 as u32,-1.0);

        //Background!
        let transform_2 = render_state.transform_buffer().add_matrix(&s_2_w);
        let second_sprite = Sprite::new([500,500],[0,0],[500,500],1,transform_2 as u32,-2.0); 

        let mut possum_entity = Entity::new();
        possum_entity.add_sprite(render_state, sprite);

        let mut bg_entity = Entity::new();
        bg_entity.add_sprite(render_state, second_sprite);

        self.entities.push(possum_entity);
        self.entities.push(bg_entity);

    }

    pub fn update(&mut self)
    {
        self.game_state.update(&self.input_state);
        //TODO: also update render state?
    }

    pub fn render(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.clear_context();
        render_state.submit_transform_buffer_uniforms();

        let mut ranges = Vec::<Range<i32>>::new();
        for entity in &self.entities
        {
            ranges.extend(entity.get_active_sprite_ranges().clone());
        }

        render_state.draw_buffer::<Sprite>(&ranges);
    }

    pub fn process_event(&self, code : &str)
    {
        self.input_state.process_input(code);
    }
}
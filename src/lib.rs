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
use graphics::sprite::{Sprite,SpriteConfig};

#[wasm_bindgen]
pub struct Game
{
    game_state: GameState,
    render_state: Option<RenderState>,
    input_state: InputState,
}

#[wasm_bindgen]
impl Game
{
    pub fn new(document: &Document) -> Self
    {
        Self
        {
            game_state: GameState::new(),
            render_state: RenderState::new(document),
            input_state: InputState::new(),
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

    pub fn load_texture(&mut self, index: u32, img: HtmlImageElement)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.load_texture(index,img);
    }

    pub fn init_render_data(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.submit_camera_uniforms(); //TODO: if we change perspective, do this more than once.
    }

    pub fn init_game_data(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        let possum_sprite_1 = SpriteConfig::new([0,0],[38,17],0,-0.5);
        let possum_sprite_2 = SpriteConfig::new([0,0],[38,17],0,-0.5);
        let bg_sprite = SpriteConfig::new([0,0],[153,119],1,-1.0); 

        self.game_state.create_entity(render_state, &vec![possum_sprite_1,possum_sprite_2]);
        self.game_state.create_entity(render_state, &vec![bg_sprite]);

        let game_state = &mut self.game_state;

        {
            let mut poss_entity = match game_state.get_mutable_entity(0)
            {
                Some(p) => p,
                None => {return}
            };

            let mut scale_down = glm::Mat4::identity().into();
            let scale = glm::vec3(0.25,0.25,0.0);
            scale_down = glm::scale(&scale_down,&scale);
            poss_entity.transform(&scale_down);
        }

        let mut bg_entity = match game_state.get_mutable_entity(1)
        {
            Some(p) => p,
            None => {return}
        };

        let mut scale_up= glm::Mat4::identity().into();
        let sc = glm::vec3(1.0,1.0,0.0);
        scale_up= glm::scale(&scale_up,&sc);

        bg_entity.transform(&scale_up)

    }

    pub fn update(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        self.game_state.update(render_state, &self.input_state);
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
        render_state.draw_buffer::<Sprite>(&self.game_state.get_active_sprite_ranges());
    }

    pub fn process_event(&self, code : &str)
    {
        self.input_state.process_input(code);
    }
}
use graphics::renderable::Renderable;
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
use util::logging::log;

#[wasm_bindgen]
pub struct Game
{
    game_state: GameState,
    render_state: Option<RenderState>,
    input_state: InputState
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
            input_state: InputState::new()
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

    pub fn init_game_data(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        self.game_state.init(render_state);
    }

    pub fn update(&mut self, delta_time: f32)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        self.game_state.update(render_state, &self.input_state, delta_time);
    }

    pub fn render(&mut self)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.clear_context();
        render_state.submit_camera_uniforms(); 
        render_state.bind_and_update_transform_buffer_data();

        render_state.draw(self.game_state.get_background_renderables()); //TODO: just sprites for now.
        render_state.draw(self.game_state.get_actor_renderables());
    }

    pub fn process_keypress_event(&mut self, pressed: bool, code : &str)
    {
        self.input_state.process_input(pressed,code);
    }
}
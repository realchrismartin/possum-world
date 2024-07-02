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
use crate::state::render_state;
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

        self.game_state.update(render_state, &mut self.input_state, delta_time);
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

        //Copy data into a batch to be drawn
        let batch = self.game_state.get_renderable_batch();

        //Draw everything in the batch.
        batch.draw(&render_state);
    }

    pub fn process_keypress_event(&mut self, pressed: bool, code : &str)
    {
        self.input_state.process_input(pressed,code);
    }

    pub fn process_click_event(&mut self, start_or_end: bool, x: i32, y: i32)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        self.input_state.process_click(start_or_end,x,render_state.get_world_size_y() as i32 - y);
    }

    pub fn process_mouse_move_event(&mut self, x: i32, y: i32)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        self.input_state.process_mouse_move(x,render_state.get_world_size_y() as i32 - y);
    }

    pub fn set_canvas_dimensions(&mut self, x: u32, y: u32)
    {
        let render_state = match &mut self.render_state
        {
            Some(r) => {r}
            None => { return; }
        };

        render_state.set_canvas_dimensions(x,y);
        self.game_state.set_world_size(render_state);
    }
}
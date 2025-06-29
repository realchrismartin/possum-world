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
mod system;
mod component;
mod scene;
mod networking;

use web_sys::{Document, HtmlImageElement};

use state::input_state::InputState;
use state::render_state::RenderState;
use scene::scene::Scene;
use networking::server_connection::ServerConnection;
use networking::message::Message;
use system::system::{init_scene, init_render_data_from_scene, run_systems};

#[wasm_bindgen]
pub struct Game
{
    scene: Scene,
    render_state: RenderState,
    input_state: InputState,
    server_connection: ServerConnection
}

#[wasm_bindgen]
impl Game
{
    pub fn new(document: &Document) -> Self
    {
        Self
        {
            scene: Scene::new(),
            render_state: RenderState::new(document),
            input_state: InputState::new(),
            server_connection: ServerConnection::new()
        }
    }

    pub fn load_shader(&mut self, vert_shader: &str, frag_shader: &str)
    {
        self.render_state.set_shader(vert_shader, frag_shader);
    }

    pub fn load_texture(&mut self, index: u32, img: HtmlImageElement)
    {
        self.render_state.load_texture(index,img);
    }

    pub fn init(&mut self)
    {
        init_scene(&mut self.scene);
        init_render_data_from_scene(&mut self.scene, &mut self.render_state)
    }

    pub fn run_systems(&mut self, delta_time: f32)
    {
        run_systems(&mut self.scene, &mut self.render_state,&mut self.input_state, &mut self.server_connection, delta_time);
    }

    pub fn process_keypress_event(&mut self, pressed: bool, code : &str)
    {
        self.input_state.process_input(pressed,code);
    }

    pub fn process_click_event(&mut self, start_or_end: bool, x: i32, y: i32)
    {
        self.input_state.process_click(start_or_end,x,y);
    }

    pub fn process_mouse_move_event(&mut self, x: i32, y: i32)
    {
        self.input_state.process_mouse_move(x,y);
    }

    pub fn set_canvas_dimensions(&mut self, x: u32, y: u32)
    {
        self.render_state.set_canvas_dimensions(x,y);
        self.input_state.set_canvas_dimensions(x,y);
    }

    pub fn send_chat_message(&mut self, content: String)
    {
        let message = Message::new_chat_message(content);
        self.server_connection.immediately_send_message(&message);
    }
}
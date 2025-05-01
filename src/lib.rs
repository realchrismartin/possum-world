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
mod system;
mod component;
mod scene;

use web_sys::{Document, HtmlImageElement};

use state::game_state::GameState;
use state::input_state::InputState;
use state::render_state::RenderState;
use scene::scene::Scene;
use crate::system::system::run_systems;
use crate::component::physics_component::PhysicsComponent;

#[wasm_bindgen]
pub struct Game
{
    game_state: GameState,
    render_state: RenderState,
    input_state: InputState,
    scene: Scene
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
            scene: Scene::new()
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

    pub fn init_game_data(&mut self)
    {
        self.render_state.clear();
        self.game_state.init(&mut self.render_state);

        /*
        //TODO: remove this placeholder stuff.
        let entity = match self.scene.add_entity()
        {
            Some(e) => e,
            None => {return;}
        };

        self.scene.add_component::<PhysicsComponent>(entity);

        for entity in self.scene.get_entities_with_components::<PhysicsComponent,OtherComponent>()
        {
            let pc = self.scene.get_component::<PhysicsComponent>(entity);
        }
        */
    }

    pub fn run_systems(&mut self, delta_time: f32)
    {
        run_systems(&mut self.game_state,&mut self.render_state,&mut self.input_state, delta_time);
    }

    pub fn process_keypress_event(&mut self, pressed: bool, code : &str)
    {
        self.input_state.process_input(pressed,code);
    }

    pub fn process_click_event(&mut self, start_or_end: bool, x: i32, y: i32)
    {
        self.input_state.process_click(start_or_end,x,self.render_state.get_canvas_size_y() as i32 - y);
    }

    pub fn process_mouse_move_event(&mut self, x: i32, y: i32)
    {
        self.input_state.process_mouse_move(x,self.render_state.get_canvas_size_y() as i32 - y);
    }

    pub fn set_canvas_dimensions(&mut self, x: u32, y: u32)
    {
        self.render_state.clear();
        self.render_state.set_canvas_dimensions(x,y);
        self.game_state.init(&mut self.render_state);
    }
}
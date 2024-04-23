use wasm_bindgen::prelude::*;
/*
use std::panic;
extern crate console_error_panic_hook;
panic::set_hook(Box::new(console_error_panic_hook::hook));
 */

mod system;
mod state;
mod util;

use state::game_state::GameState;
use state::input_state::InputState;
use state::render_state::RenderState;

use system::system::update_game;
use system::system::render_game;

#[wasm_bindgen]
pub fn new_game_state() -> GameState
{
    GameState::new()
}

#[wasm_bindgen]
pub fn new_render_state() -> RenderState 
{
    RenderState::new()
}

#[wasm_bindgen]
pub fn new_input_state() -> InputState
{
    InputState::new()
}

#[wasm_bindgen]
pub fn update(game_state : &mut GameState, input_state : &InputState)
{
    update_game(game_state,input_state);
}

#[wasm_bindgen]
pub fn render(game_state : &GameState, render_state: &RenderState)
{
    render_game(game_state, render_state);
}

#[wasm_bindgen]
pub fn process_event(input_state : &mut InputState, code : &str)
{
    input_state.process_input(code);
}
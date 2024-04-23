use crate::util::logging::log;

use crate::state::game_state::GameState;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

pub fn render_game(game_state : &GameState, render_state : &RenderState)
{
    /*
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas= canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;
        */

    log("Rendered!");
}

pub fn update_game(game_state : &mut GameState, input_state : &InputState)
{
    log("Updated.");
}

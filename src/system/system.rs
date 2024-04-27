use crate::state::game_state::GameState;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

pub fn render_game(game_state : &GameState, render_state : &mut RenderState)
{
    render_state.test_submit_data_and_draw();
}

pub fn update_game(game_state : &mut GameState, input_state : &InputState)
{
    //log("Updated.");
}

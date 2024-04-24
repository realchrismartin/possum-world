use crate::wasm_bindgen;
use crate::util::logging::log;
use crate::state::input_state::InputState;

#[wasm_bindgen]
pub struct GameState
{

}

#[wasm_bindgen]
impl GameState
{
    pub fn new() -> Self
    {
        Self
        {

        }
    }

    pub fn update(&mut self, input_state: &InputState)
    {
        log("Updated the game!");
    }
}
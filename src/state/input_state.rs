use crate::util::logging::log;
use crate::wasm_bindgen;


#[wasm_bindgen]
pub struct InputState
{
}

#[wasm_bindgen]
impl InputState
{
    pub fn new() -> Self
    {
        Self
        {
        }
    }

    pub fn process_input(self: &Self, code: &str)
    {
        match code{
            "KeyW" => {
                log("Key: W");
            },
            "KeyS" => {
                log("S");
            },
            "KeyA" => {
                log("A");
            },
            "KeyD" => {
                log("D");
            },
            _ => {}
        }
    }
}
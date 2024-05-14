use crate::util::logging::log;
use crate::wasm_bindgen;
use std::collections::HashMap;


#[wasm_bindgen]
pub struct InputState
{
    pressed: HashMap<KeyPress,bool>
}

enum KeyPress
{
    W,S,A,D
}

#[wasm_bindgen]
impl InputState
{
    pub fn new() -> Self
    {
        Self
        {
            pressed: HashMap::new()
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
use crate::util::logging::log;
use crate::wasm_bindgen;
use std::collections::HashMap;
use std::fmt;

#[wasm_bindgen]
pub struct InputState
{
    active: HashMap<KeyPress,bool>
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
enum KeyPress
{
    W,S,A,D
}

impl fmt::Display for KeyPress
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self 
        {
            KeyPress::W => write!(f, "W"),
            KeyPress::S => write!(f, "S"),
            KeyPress::A => write!(f, "A"),
            KeyPress::D => write!(f, "D"),
        }
    }
}

#[wasm_bindgen]
impl InputState
{
    pub fn new() -> Self
    {
        Self
        {
            active: HashMap::from([(KeyPress::W, false),(KeyPress::S, false),(KeyPress::A, false), (KeyPress::D, false)])
        }
    }

    pub fn process_input(&mut self, pressed: bool, code: &str)
    {
        match code
        {
            "KeyW" => 
            {
                self.active.insert(KeyPress::W,pressed);
            },
            "KeyS" => 
            {
                self.active.insert(KeyPress::S,pressed);
            },
            "KeyA" => 
            {
                self.active.insert(KeyPress::A,pressed);
            },
            "KeyD" => 
            {
                self.active.insert(KeyPress::D,pressed);
            },
            _ => {}
        }

        //TODO: debugging
        let mut debug_output_string = "".to_owned();

        for (k,v) in &self.active
        {
            if *v
            {
                debug_output_string.push_str(k.to_string().as_str());
            }
        }
        
        if debug_output_string.len() > 0
        {
            log(&debug_output_string.as_str());
        }

    }
}
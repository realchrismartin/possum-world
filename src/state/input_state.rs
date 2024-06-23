use crate::util::logging::log;
use std::collections::HashMap;
use std::fmt;

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

impl InputState
{
    pub fn new() -> Self
    {
        Self
        {
            active: HashMap::from([(KeyPress::W, false),(KeyPress::S, false),(KeyPress::A, false), (KeyPress::D, false)])
        }
    }

    pub fn is_pressed(&self, key : KeyPress) -> bool
    {
        let res = match self.active.get(&key)
        {
            Some(r) => return *r,
            None => {}
        };

        false
    }

    pub fn get_movement_direction(&self) -> glm::Vec2
    {
        let mut res = glm::vec2(0.0,0.0);

        if self.is_pressed(KeyPress::W)
        {
            res += glm::vec2(0.0,1.0);
        }

        if self.is_pressed(KeyPress::S)
        {
            res += glm::vec2(0.0,-1.0);
        }

        if self.is_pressed(KeyPress::A)
        {
            res += glm::vec2(-1.0,0.0);
        }

        if self.is_pressed(KeyPress::D)
        {
            res += glm::vec2(1.0,0.0);
        }

        res
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
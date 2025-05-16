use crate::util::logging::log;
use std::collections::{HashMap, VecDeque};
use std::fmt;

#[derive(Copy)]
#[derive(Clone)]
pub struct Click
{
    x_coordinate: i32,
    y_coordinate: i32,
    canvas_ratio_x: f32,
    canvas_ratio_y: f32,
    active: bool
}

impl Click
{
    pub fn new() -> Self
    {
        Self
        {
            x_coordinate: 0,
            y_coordinate: 0,
            canvas_ratio_x: 0.0,
            canvas_ratio_y: 0.0,
            active: false
        }
    }

    pub fn set_x_coordinate(&mut self, x: i32)
    {
        self.x_coordinate = x;
    }

    pub fn set_y_coordinate(&mut self, y: i32)
    {
        self.y_coordinate = y;
    }

    pub fn set_canvas_ratio_x(&mut self, x: f32)
    {
        self.canvas_ratio_x = x;
    }

    pub fn set_canvas_ratio_y(&mut self, y: f32)
    {
        self.canvas_ratio_y = y;
    }

    pub fn set_active(&mut self, active : bool)
    {
        self.active = active;
    }

    pub fn get_canvas_ratio_x(&self) -> &f32
    {
        &&self.canvas_ratio_x
    }

    pub fn is_active(&self) -> bool
    {
        self.active
    }
}

pub struct InputState
{
    active: HashMap<KeyPress,bool>,
    click_locations: VecDeque<Click>,
    last_mouse_location: Click,
    canvas_size_x: u32,
    canvas_size_y: u32,
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum KeyPress
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
            active: HashMap::from([(KeyPress::W, false),(KeyPress::S, false),(KeyPress::A, false), (KeyPress::D, false)]),
            click_locations: VecDeque::new(),
            last_mouse_location: Click::new(),
            canvas_size_x: 1,
            canvas_size_y: 1,
        }
    }

    pub fn has_next_click(&self) -> bool
    {
        self.click_locations.len() > 0
    }
    pub fn get_next_click(&mut self) -> Option<Click>
    {
        let f = self.click_locations.pop_front();

        match f
        {
            Some(c) => 
            {
                return Some(c);
            },
            None => {
            }
        };

        None
    }

    pub fn get_current_mouse_location(&self) -> &Click
    {
        &self.last_mouse_location
    }

    pub fn is_pressed(&self, key : KeyPress) -> bool
    {
        match self.active.get(&key)
        {
            Some(r) => return *r,
            None => {}
        };

        false
    }

    pub fn process_click(&mut self, start_or_end_click: bool, x: i32, y: i32)
    {
        let inverted_y = self.canvas_size_y as i32 - y;

        //Started click
        if start_or_end_click
        {
            self.last_mouse_location.set_active(true);
            self.last_mouse_location.set_x_coordinate(x);
            self.last_mouse_location.set_y_coordinate(inverted_y);
            self.last_mouse_location.set_canvas_ratio_x(x as f32 / self.canvas_size_x as f32);
            self.last_mouse_location.set_canvas_ratio_y(inverted_y as f32 / self.canvas_size_y as f32);

            self.click_locations.push_back(self.last_mouse_location.clone());
            return;
        }

        //Ended click
        self.last_mouse_location.set_active(false);
    }

    pub fn process_mouse_move(&mut self, x: i32, y: i32)
    {
        let inverted_y = self.canvas_size_y as i32 - y;

        self.last_mouse_location.set_x_coordinate(x);
        self.last_mouse_location.set_y_coordinate(inverted_y);
        self.last_mouse_location.set_canvas_ratio_x(x as f32 / self.canvas_size_x as f32);
        self.last_mouse_location.set_canvas_ratio_y(inverted_y as f32 / self.canvas_size_y as f32);
    }

    pub fn set_canvas_dimensions(&mut self, canvas_size_x: u32, canvas_size_y: u32)
    {
        self.canvas_size_x = canvas_size_x;
        self.canvas_size_y = canvas_size_y;
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
use crate::state::render_state::RenderState;
use crate::graphics::renderable::{Renderable, RenderableConfig};
use crate::graphics::sprite::Sprite;
use std::collections::VecDeque;
use crate::util::logging::log;

pub struct AnimatedEntity
{
    active_sprite: Vec<Sprite>,
    inactive_sprites_left: VecDeque<Sprite>,
    inactive_sprites_right: VecDeque<Sprite>,
    shared_transform_index: u32,
    animating: bool,
    facing_right: bool,
    time_since_frame_change: f32,
    time_per_frame: f32
}

impl AnimatedEntity
{
    pub fn new(render_state: &mut RenderState, time_per_frame: f32,
        left_sprite_configs: Vec<RenderableConfig>,
        right_sprite_configs: Vec<RenderableConfig>,
        facing_right: bool
    ) -> Self
    {
        //No sprites? Nothing to set up.
        if left_sprite_configs.len() <= 0 && right_sprite_configs.len() <= 0
        {
            return Self
            {
                active_sprite: Vec::new(),
                inactive_sprites_left: VecDeque::new(),
                inactive_sprites_right: VecDeque::new(),
                shared_transform_index: 0, //Will not be accurate
                animating: false,
                facing_right: true,
                time_since_frame_change: 0.0,
                time_per_frame: time_per_frame
            }
        }

        let mut active_sprites = Vec::new();
        let mut left_inactive_sprites = VecDeque::<Sprite>::new();
        let mut right_inactive_sprites = VecDeque::<Sprite>::new();

        //Create all of the sprites but only use one transform index.
        let transform = render_state.request_new_transform();

        for config in &left_sprite_configs
        {
            let sprite = match render_state.request_new_renderable_with_existing_transform::<Sprite>(&config,transform)
            {
                Some(s) => s,
                None => { continue; }
            };

            left_inactive_sprites.push_back(sprite);
        }

        for config in &right_sprite_configs
        {
            let sprite = match render_state.request_new_renderable_with_existing_transform::<Sprite>(&config,transform)
            {
                Some(s) => s,
                None => { continue; }
            };

            right_inactive_sprites.push_back(sprite);
        }

        //Now select an active sprite 

        if facing_right
        {
            let active = match right_inactive_sprites.pop_front()
            {
                Some(sprite) => sprite,
                None => {
                    log("Tried to initialize a right facing sprite with no viable right facing configs");
                    return Self
                    {
                        active_sprite: Vec::new(),
                        inactive_sprites_left: VecDeque::new(),
                        inactive_sprites_right: VecDeque::new(),
                        shared_transform_index: 0, //Will not be accurate
                        animating: false,
                        facing_right: true,
                        time_since_frame_change: 0.0,
                        time_per_frame: time_per_frame
                    }
                }
            };

            active_sprites.push(active);

        } else 
        {
            let active = match left_inactive_sprites.pop_front()
            {
                Some(sprite) => sprite,
                None => {
                    log("Tried to initialize a left facing sprite with no viable left facing configs");
                    return Self
                    {
                        active_sprite: Vec::new(),
                        inactive_sprites_left: VecDeque::new(),
                        inactive_sprites_right: VecDeque::new(),
                        shared_transform_index: 0, //Will not be accurate
                        animating: false,
                        facing_right: true,
                        time_since_frame_change: 0.0,
                        time_per_frame: time_per_frame
                    }
                }
            };

            active_sprites.push(active);
        }

        Self
        {
            active_sprite: active_sprites,
            inactive_sprites_left: left_inactive_sprites,
            inactive_sprites_right: right_inactive_sprites,
            shared_transform_index: transform,
            animating: false,
            time_since_frame_change: 0.0,
            time_per_frame,
            facing_right
        }
    }

    pub fn get_facing_right(&self) -> bool
    {
        self.facing_right
    }


    pub fn set_facing(&mut self, face_right: bool)
    {
        if self.facing_right == face_right
        {
            //Do nothing
            return;
        }

        if self.facing_right && self.inactive_sprites_left.len() == 0
        {
            //Don't allow a flip if the other side has no sprites
            return; 
        }

        if !self.facing_right && self.inactive_sprites_right.len() == 0
        {
            //Don't allow a flip if the other side has no sprites
            return;
        }

        //Return the active sprite back to the correct inactive queue
        //Assumes there is only one active sprite = TODO
        if self.facing_right && self.active_sprite.len() > 0
        {
           let active = self.active_sprite.pop(); //Front is the same as back
           self.inactive_sprites_right.push_back(active.unwrap());
        } else if !self.facing_right && self.active_sprite.len() > 0
        {
           let active = self.active_sprite.pop(); //Front is the same as back
           self.inactive_sprites_left.push_back(active.unwrap());
        }

        //Grab the next sprite from the new queue and make it active
        self.facing_right = face_right;

        if self.facing_right
        {
            let new_active = self.inactive_sprites_right.pop_front();
            self.active_sprite.push(new_active.unwrap());
        } else
        {
            let new_active = self.inactive_sprites_left.pop_front();
            self.active_sprite.push(new_active.unwrap());
        }

    }

    pub fn step_animation(&mut self)
    {
        if self.active_sprite.len() <= 0 || self.active_sprite.len() > 1
        {
            return;
        }

        if self.facing_right && self.inactive_sprites_right.len() == 0
        {
            return;
        }

        if !self.facing_right && self.inactive_sprites_left.len() == 0
        {
            return;
        }

        //Pop the back. This is also the front. (we want the front item)
        let active_sprite = match self.active_sprite.pop()
        {
            Some(sprite) => sprite,
            None => { return; }
        };

        //Put the active sprite in the inactive queue at the back
        if self.facing_right
        {
            self.inactive_sprites_right.push_back(active_sprite);

            //Pop the front of the queue
            let new_active_sprite = match self.inactive_sprites_right.pop_front()
            {
                Some(sprite) => sprite,
                None => { return; }
            };

            //Make it active
            self.active_sprite.push(new_active_sprite);
        } else 
        {
            self.inactive_sprites_left.push_back(active_sprite);

            //Pop the front of the queue
            let new_active_sprite = match self.inactive_sprites_left.pop_front()
            {
                Some(sprite) => sprite,
                None => { return; }
            };

            //Make it active
            self.active_sprite.push(new_active_sprite);
        }

    }

    pub fn update(&mut self, delta_time: f32)
    {
        if !self.animating
        {
            return;
        }

        self.time_since_frame_change += delta_time;

        if self.time_since_frame_change >= self.time_per_frame
        {
            self.step_animation();
            self.time_since_frame_change = 0.0;
        }
    }

    pub fn set_animating(&mut self, state: bool)
    {
        self.animating = state;
    }

    pub fn get_active_sprite(&self) -> &Vec<Sprite>
    {
        &self.active_sprite
    }

    pub fn get_transform_location(&self) -> u32
    {
        self.shared_transform_index
    }

}
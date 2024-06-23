use crate::state::render_state::RenderState;
use crate::graphics::renderable::{Renderable, RenderableConfig};
use crate::graphics::sprite::Sprite;
use std::collections::VecDeque;

pub struct AnimatedEntity
{
    active_sprite: Vec<Sprite>,
    inactive_sprites: VecDeque<Sprite>,
    shared_transform_index: u32,
    animating: bool,
    time_since_frame_change: f32,
    time_per_frame: f32
}

impl AnimatedEntity
{
    pub fn new(render_state: &mut RenderState, time_per_frame: f32, sprite_configs: Vec<RenderableConfig>) -> Self
    {
        //Create all of the sprites but only use one transform index.
        let mut transform : Option<u32> = None;
        let mut active_sprites = Vec::new();
        let mut inactive_sprites = VecDeque::new();

        let mut first = true;
        for config in &sprite_configs
        {
            if first
            {
                let sprite = match render_state.request_new_renderable::<Sprite>(&config)
                {
                    Some(s) => s,
                    None => { continue; }
                };

                transform = Some(sprite.get_transform_location());

                active_sprites.push(sprite);

                first = false;
            } else 
            {
                let sprite = match render_state.request_new_renderable_with_existing_transform::<Sprite>(&config,transform.unwrap())
                {
                    Some(s) => s,
                    None => { continue; }
                };

                inactive_sprites.push_back(sprite);
            }
        }

        if sprite_configs.len() <= 0
        {
            transform = Some(0); //Prevent a panic if there's no sprite configs provided.
        }

        Self
        {
            active_sprite: active_sprites,
            inactive_sprites,
            shared_transform_index: transform.unwrap(), //will panic if not set
            animating: false,
            time_since_frame_change: 0.0,
            time_per_frame
        }
    }

    pub fn step_animation(&mut self)
    {
        if self.active_sprite.len() <= 0 || self.active_sprite.len() > 1
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
        self.inactive_sprites.push_back(active_sprite);

        //Pop the front of the queue
        let new_active_sprite = match self.inactive_sprites.pop_front()
        {
            Some(sprite) => sprite,
            None => { return; }
        };

        //Make it active
        self.active_sprite.push(new_active_sprite);
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
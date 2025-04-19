use crate::state::render_state::RenderState;
use crate::graphics::renderable::{Renderable,RenderableConfig};
use crate::graphics::sprite::Sprite;
use crate::util::logging::log;

pub struct AnimatedEntity
{
    sprite_index: usize,
    sprites_left: Vec<Sprite>,
    sprites_left_sizes: Vec<i32>,
    sprites_right: Vec<Sprite>,
    sprites_right_sizes: Vec<i32>,
    shared_transform_index: Option<u32>,
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
    ) -> Option<Self>
    {
        //No sprites? Nothing to set up.
        if left_sprite_configs.len() <= 0 && right_sprite_configs.len() <= 0
        {
            return None;
        }

        let mut left_sprites = Vec::<Sprite>::new();
        let mut right_sprites = Vec::<Sprite>::new();
        let mut left_sprites_sizes = Vec::<i32>::new();
        let mut right_sprites_sizes = Vec::<i32>::new();

        //Create all of the sprites but only use one transform index.
        let transform = render_state.request_new_transform();

        for config in &left_sprite_configs
        {
            let sprite = match render_state.request_new_renderable_with_existing_transform::<Sprite>(&config,transform)
            {
                Some(s) => s,
                None => { continue; }
            };
            left_sprites.push(sprite);

            let size = config.get_size();
            left_sprites_sizes.push(size[0]);
            left_sprites_sizes.push(size[1]);
        }

        for config in &right_sprite_configs
        {
            let sprite = match render_state.request_new_renderable_with_existing_transform::<Sprite>(&config,transform)
            {
                Some(s) => s,
                None => { continue; }

            };

            right_sprites.push(sprite);

            let size = config.get_size();
            right_sprites_sizes.push(size[0]);
            right_sprites_sizes.push(size[1]);
        }

        if facing_right && right_sprites.len() <= 0
        {
            return None;
        }

        if !facing_right && left_sprites.len() <= 0
        {
            return None;
        }

        Some(Self
        {
            sprite_index: 0,
            sprites_left: left_sprites,
            sprites_right: right_sprites,
            sprites_left_sizes: left_sprites_sizes,
            sprites_right_sizes: right_sprites_sizes,
            shared_transform_index: Some(transform),
            animating: false,
            time_since_frame_change: 0.0,
            time_per_frame,
            facing_right
        })
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

        if self.facing_right && self.sprites_left.len() == 0
        {
            //Don't allow a flip if the other side has no sprites
            return; 
        }

        if !self.facing_right && self.sprites_right.len() == 0
        {
            //Don't allow a flip if the other side has no sprites
            return;
        }

        self.facing_right = face_right;
        self.sprite_index = 0;
    }

    pub fn step_animation(&mut self)
    {
        if self.facing_right && self.sprites_right.len() == 0
        {
            return;
        }

        if !self.facing_right && self.sprites_left.len() == 0
        {
            return;
        }

        let mut sprite_index = self.sprite_index;

        sprite_index += 1;


        if self.facing_right
        {
            sprite_index = sprite_index % self.sprites_right.len();
        } else 
        {
            sprite_index = sprite_index % self.sprites_left.len();
        }

        self.sprite_index = sprite_index;

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

    pub fn get_renderable(&self) -> Option<&Sprite>
    {
        if self.facing_right
        {
            return self.sprites_right.get(self.sprite_index);
        }

        self.sprites_left.get(self.sprite_index)
    }

    pub fn get_transform_location(&self) -> Option<u32>
    {
        self.shared_transform_index
    }

    fn get_current_sprite_size(&self) -> &[i32]
    {
        if self.facing_right
        {
            return &self.sprites_right_sizes[self.sprite_index..self.sprite_index+1]
        }

        return &self.sprites_left_sizes[self.sprite_index..self.sprite_index+1]
    }

    pub fn get_scaled_size(&self, render_state: &RenderState) -> Option<glm::Vec3>
    {
        let index = match self.get_transform_location()
        {
            Some(s) => s,
            None => { return None; }
        };

        let scale = match render_state.get_scale_with_index(index)
        {
            Some(s) => s,
            None => { return None; }
        };

        let size_x = self.sprites_right_sizes[self.sprite_index] as f32;
        let size_y = self.sprites_right_sizes[self.sprite_index+1] as f32;

        let world_size_x = render_state.get_world_size_x() as f32;
        let world_size_y = render_state.get_world_size_y() as f32;

        //Assume Y is 1.0 (exact view height), x is less than that
        let mut current_pixel_size_x = (size_x / size_y) * world_size_x * scale.x;
        let mut current_pixel_size_y = world_size_y * scale.y;

        //Use the sizes to infer the dimensions of the sprite on the canvas
        if(size_x > size_y) 
        {
            //X is 1.0 (exact view width), y is less than that
            current_pixel_size_x = world_size_x * scale.x;
            current_pixel_size_y = (size_y / size_x) * world_size_y * scale.y
        }

        Some(glm::vec3(current_pixel_size_x, current_pixel_size_y, 1.0))
    }
}
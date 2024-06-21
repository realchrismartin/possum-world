use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;
use crate::game::entity::Entity;
use crate::graphics::sprite::{Sprite,SpriteConfig};
use std::ops::Range;
use crate::util::logging::log;

pub struct GameState
{
    entities: Vec<Entity>
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            entities: Vec::new()
        }
    }

    pub fn update(&mut self, input_state: &InputState)
    {
        //log("Updated the game!");
    }

    pub fn create_entity(&mut self, render_state: &mut RenderState, sprite_configs: &Vec<SpriteConfig>)
    {
        let mut entity = Entity::new();

        for config in sprite_configs
        {
            let texture_dimensions = match render_state.get_texture(config.get_texture_index())
            {
                Some(t) => t.get_dimensions(),
                None => { continue; }
            };

            let transform_index = render_state.transform_buffer().add_matrix(&glm::Mat4::identity().into());
            let sprite = Sprite::new(config,transform_index as u32,texture_dimensions);
            let range = render_state.submit_data(&sprite);
            entity.add_sprite(sprite, range);
        }

        self.entities.push(entity);
    }

    pub fn get_active_sprite_ranges(&self) -> Vec<Range<i32>>
    {
        //TODO: performance 
        let mut ranges = Vec::<Range<i32>>::new();
        for entity in &self.entities
        {
            ranges.extend(entity.get_active_sprite_ranges().clone());
        }

        ranges
    }


}
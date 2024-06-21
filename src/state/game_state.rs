use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;
use crate::game::entity::Entity;
use crate::graphics::sprite::{Sprite,SpriteConfig};
use std::ops::Range;
use crate::util::logging::log;
use crate::util::logging::log_f32;

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

    pub fn update(&mut self, render_state: &mut RenderState, input_state: &InputState)
    {
        //log("Updated the game!");

        //After all transformations are done:
        for entity in &mut self.entities
        {
            //Apply the entity's transform by replacing all transforms the entity "owns" in the transform buffer.
            if entity.is_transform_dirty()
            {
                log("dirt!");
                for transform_index in entity.get_transform_indices()
                {
                    log_f32(transform_index as f32);
                    let transform = entity.get_transform();
                    render_state.transform_buffer().update_matrix(transform_index, transform);
                }

                entity.set_transform_clean();
            }
        }
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

    pub fn get_mutable_entity(&mut self, index: u32) -> Option<&mut Entity>
    {
        if index >= self.entities.len() as u32
        {
            return None
        }

        Some(&mut self.entities[index as usize])
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
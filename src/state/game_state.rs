use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::{Renderable,RenderableConfig};

pub struct GameState
{
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
        }
    }

    pub fn update(&mut self, render_state: &mut RenderState, input_state: &InputState)
    {
        //log("Updated the game!");

        //After all transformations are done:
        /*
        for entity in &mut self.entities
        {
            //Apply the entity's transform by replacing all transforms the entity "owns" in the transform buffer.
            let mut transform = entity.get_transform();

            if transform.dirty()
            {
                render_state.transform_buffer().update_transform(transform_index, transform);
                transform.set_clean();
            }
        }
         */
    }

}
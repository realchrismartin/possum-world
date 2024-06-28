use crate::graphics::sprite::Sprite;
use crate::state::render_state::RenderState;

//TODO: add other renderable types here.

pub struct DrawBatch
{
    sprites: Vec<Sprite>
}

impl DrawBatch
{
    pub fn new() -> Self
    {
        Self 
        {
            sprites: Vec::new()
        }
    }

    pub fn add_sprite(&mut self, sprite: &Sprite)
    {
        self.sprites.push(sprite.clone());
    }

    pub fn draw(&self, render_state: &RenderState)
    {
        render_state.draw(&self.sprites);
    }

}
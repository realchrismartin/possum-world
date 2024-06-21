use crate::graphics::sprite::Sprite;
use crate::graphics::renderable::Renderable;
use std::ops::Range;

pub struct Entity
{
    sprites: Vec<Sprite>,
    sprite_draw_ranges: Vec<Range::<i32>>,
    active_sprite_draw_ranges: Vec<Range::<i32>>,
}

impl Entity
{
    pub fn new() -> Self
    {
        Self
        {
            sprites: Vec::new(),
            sprite_draw_ranges: Vec::new(),
            active_sprite_draw_ranges: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite, range: Range<i32>)
    {
        //TODO!
        //sprite.set_should_be_drawn(true);
        let r2 = range.clone();

        self.sprites.push(sprite);
        self.sprite_draw_ranges.push(range);
        self.active_sprite_draw_ranges.push(r2); //Assumes sprite starts drawn
    }

    pub fn set_sprite_status(&mut self, index: usize, enabled: bool)
    {
        //TODO: efficiency
        self.active_sprite_draw_ranges.clear();

        for i in 0..self.sprites.len()
        {
            if i == index
            {
                self.sprites[i].set_should_be_drawn(enabled);
            }

            if self.sprites[i].should_be_drawn()
            {
                self.active_sprite_draw_ranges.push(self.sprite_draw_ranges[i].clone());
            }
        }
    }

    pub fn get_active_sprite_ranges(&self) -> &Vec<Range::<i32>>
    {
        &self.active_sprite_draw_ranges
    }

    //TODO: support transforming an entity's sprites using their transform indices
    //Each sprite has a transform index
    //We can use these indices to update the matrices held in the transform buffer just before the next draw
    // hold a matrix in the entity, accrue transformations, then replace each matrix in the buffer?

}


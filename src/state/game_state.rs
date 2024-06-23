use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::RenderableConfig;
use crate::graphics::sprite::Sprite;
use rand::Rng;

pub struct GameState
{
    sprites: Vec<Sprite>,
    elapsed_time: f32
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            sprites: Vec::new(),
            elapsed_time: 0.0
        }
    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {

        let possum_sprite_1 = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([0,0],[376,192],0,-0.5))
        {
            Some(s) => s,
            None => { return; }
        };

        self.sprites.push(possum_sprite_1);


        //Generate a random tile grid
        let mut rng = rand::thread_rng();

        //TODO
        let mut next_y_placement = 0.0;
        let mut next_x_placement = 0.0;

        for y in 0..9
        {
            for x in 0..9
            {
                let tex_coord = rng.gen_range(0..4) * 100;
                let tile = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([tex_coord,0],[100,100],1,0.0))
                {
                    Some(s) => s,
                    None => { return; }
                };

                render_state.set_scale(&tile,glm::vec3(0.2,0.2,1.0));
                render_state.set_translation(&tile, glm::vec3(next_x_placement,next_y_placement,0.0));
                self.sprites.push(tile);

                next_x_placement += 0.1;

                break; //TODO
            }

            next_x_placement = 0.1;
            next_y_placement += 0.1;

            break; //TODO
        }

        //render_state.set_scale(&possum_sprite_1,glm::vec3(0.3,0.3,0.1));
        //render_state.set_scale(&bg_sprite,glm::vec3(1.0,1.0,0.1));
    }

    pub fn update(&mut self, render_state: &mut RenderState, input_state: &InputState, delta_time: f32)
    {
        //TODO: process enqueued renderable requests
        //TODO: process enqueued transform requests

        //TODO: not safe, temporary for testing
        render_state.set_rotation(&self.sprites[0], ((self.elapsed_time as i32) % 360) as f32);
        render_state.set_translation(&self.sprites[0], glm::vec3(0.0,0.0,-0.5));

        //render_state.set_translation(&self.sprites[0], glm::vec3(0.0,f32::sin(self.elapsed_time / 1000.0),-0.5));

        self.elapsed_time += delta_time;
    }

    pub fn get_active_renderables(&self) -> &Vec<Sprite>
    {
        //TODO: for now, just sprites
        &self.sprites
    }

}
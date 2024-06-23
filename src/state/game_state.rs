use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::RenderableConfig;
use crate::graphics::sprite::Sprite;
use glm::U1;
use rand::Rng;

pub struct GameState
{
    sprites: Vec<Sprite>,
    player_position: glm::Vec2,
    elapsed_time: f32
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            sprites: Vec::new(),
            elapsed_time: 0.0,
            player_position: glm::vec2(500.0,500.0)
        }
    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {

        let player = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([0,0],[100,100],0,-0.5))
        {
            Some(s) => s,
            None => { return; }
        };

        render_state.set_scale(&player, glm::vec3(0.1,0.1,1.0));

        self.sprites.push(player);

        //Generate a random tile grid
        let mut rng = rand::thread_rng();

        //Since each position is the center of a tile, we offset the initial placement by a tile half width
        let mut next_y_placement = 100.0;
        let mut next_x_placement = 100.0;

        for y in 0..5
        {
            for x in 0..5
            {
                let tex_coord = rng.gen_range(0..3) * 100;
                let tile = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([tex_coord,0],[100,100],1,0.0))
                {
                    Some(s) => s,
                    None => { return; }
                };

                render_state.set_scale(&tile,glm::vec3(0.2,0.2,1.0));
                render_state.set_position(&tile, glm::vec2(next_x_placement,next_y_placement));
                self.sprites.push(tile);

                next_x_placement += 200.0;

            }

            next_x_placement = 100.0; //Resetting with a half width offset
            next_y_placement += 200.0;
        }
    }

    pub fn update(&mut self, render_state: &mut RenderState, input_state: &InputState, delta_time: f32)
    {
        //TODO: process enqueued renderable requests
        //TODO: process enqueued transform requests

        //TODO: not safe, temporary for testing
        /*
        render_state.set_rotation(&self.sprites[0], ((self.elapsed_time as i32) % 360) as f32);
        render_state.set_translation(&self.sprites[0], glm::vec3(0.0,0.0,-0.5));
         */

        //render_state.set_translation(&self.sprites[0], glm::vec3(0.0,f32::sin(self.elapsed_time / 1000.0),-0.5));

        self.elapsed_time += delta_time;

        let movement_direction = input_state.get_movement_direction();

        if movement_direction.x == 0.0 && movement_direction.y == 0.0
        {
            return;
        }

        //TODO: arbitrary distance
        self.player_position += glm::vec2(10.0 * movement_direction.x,10.0 * movement_direction.y);

        render_state.set_position(&self.sprites[0], self.player_position);
    }



    pub fn get_active_renderables(&self) -> &Vec<Sprite>
    {
        //TODO: for now, just sprites
        &self.sprites
    }

}
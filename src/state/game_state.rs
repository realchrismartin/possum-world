use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::RenderableConfig;
use crate::game::animated_entity::AnimatedEntity;
use crate::graphics::sprite::Sprite;
use rand::Rng;

pub struct GameState
{
    player: Option<AnimatedEntity>,
    sprites: Vec<Sprite>,
    player_position: glm::Vec2
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            player: None,
            sprites: Vec::new(),
            player_position: glm::vec2(500.0,200.0)
        }
    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {
        //TODO: rectangular for now because otherwise we stretch onto the rectangular base sprite.
        let player = AnimatedEntity::new(render_state,50.0,vec![
            RenderableConfig::new([0,0],[48,48],0,-0.5),
            RenderableConfig::new([48,0],[48,48],0,-0.5),
            RenderableConfig::new([96,0],[48,48],0,-0.5),
            RenderableConfig::new([144,0],[48,48],0,-0.5),
            RenderableConfig::new([192,0],[48,48],0,-0.5),
            RenderableConfig::new([240,0],[48,48],0,-0.5),
            RenderableConfig::new([288,0],[48,48],0,-0.5),
            RenderableConfig::new([336,0],[48,48],0,-0.5),
        ]);

        render_state.set_position_with_index(player.get_transform_location(), self.player_position);
        render_state.set_scale_with_index(player.get_transform_location(),glm::vec3(0.1,0.1,1.0));

        self.player = Some(player);

        //Generate a random tile grid
        let mut rng = rand::thread_rng();

        //Since each position is the center of a tile, we offset the initial placement by a tile half width
        let mut next_y_placement = 100.0;
        let mut next_x_placement = 100.0;

        let use_sprites  = vec![
            RenderableConfig::new([0,0],[100,100],1,0.0), //ground
            RenderableConfig::new([100,0],[100,100],1,0.0), //background// TODO: unused
            RenderableConfig::new([200,0],[100,100],1,0.0), //background
            RenderableConfig::new([300,0],[100,100],1,0.0), //background
        ];

        for y in 0..5
        {
            for x in 0..5
            {

                let mut used_sprite_index = 0;
                if y > 0
                {
                    used_sprite_index = rng.gen_range(1..4);
                }

                let tile = match render_state.request_new_renderable::<Sprite>(use_sprites.get(used_sprite_index).unwrap())
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

        let player = match &mut self.player
        {
            Some(p) => p,
            None => { return; }
        };

        player.update(delta_time);

        let movement_direction = input_state.get_movement_direction();

        if movement_direction.x == 0.0 && movement_direction.y == 0.0
        {
            player.set_animating(false);
            return;
        }

        player.set_animating(true);
        
        
        //TODO: arbitrary speed/ distance
        //TODO: ignoring Y movement
        self.player_position += glm::vec2((delta_time / 5.0) * movement_direction.x,0.0);

        render_state.set_position_with_index(player.get_transform_location(), self.player_position);

    }

    pub fn get_background_renderables(&self) -> &Vec<Sprite>
    {
        //TODO: for now, just sprites
        &self.sprites
    }

    pub fn get_player_renderables(&self) -> &Vec<Sprite>
    {
        let player = match &self.player
        {
            Some(p) => p,
            None => { return &self.sprites; } //TODO: fix this
        };

        player.get_active_sprite()
    }

}
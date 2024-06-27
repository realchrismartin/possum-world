use crate::graphics::transform;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::RenderableConfig;
use crate::game::animated_entity::AnimatedEntity;
use crate::graphics::sprite::Sprite;
use rand::Rng;
use crate::util::logging::log;

pub struct GameState
{
    friendly_possums: Vec<AnimatedEntity>,
    tiles: Vec<Sprite>
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            friendly_possums: Vec::new(),
            tiles: Vec::new()
        }
    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {

       let mut rng = rand::thread_rng();

       for i in 0..5
       {
            let mut y = rng.gen_range(300..500);
            let mut x = rng.gen_range(50..900);

            if i == 0
            {
                x = 500;
                y = 100;
            }

           let poss = match Self::add_possum(render_state,glm::vec2(x as f32,y as f32))
           {
                   Some(p) => p,
                   None => { return; }
           };

           self.friendly_possums.push(poss);
       }

       self.init_tile_grid(render_state);
    }

    pub fn init_tile_grid(&mut self, render_state: &mut RenderState)
    {
        //Generate a random tile grid
        let mut rng = rand::thread_rng();

        //Since each position is the center of a tile, we offset the initial placement by a tile half width
        let mut next_y_placement = 50.0;
        let mut next_x_placement = 50.0;

        let use_sprites  = vec![
            RenderableConfig::new([0,0],[100,100],1,0.0), //ground
            RenderableConfig::new([100,0],[100,100],1,0.0), //background
            RenderableConfig::new([200,0],[100,100],1,0.0), //background
            RenderableConfig::new([300,0],[100,100],1,0.0), //background
        ];

        for y in 0..10
        {
            for x in 0..10
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

                render_state.set_scale(&tile,glm::vec3(0.1,0.1,1.0));
                render_state.set_position(&tile, glm::vec2(next_x_placement,next_y_placement));
                self.tiles.push(tile);

                next_x_placement += 100.0;
            }

            next_x_placement = 50.0; //Resetting with a half width offset
            next_y_placement += 100.0;
        }
    }

    pub fn add_possum(render_state: &mut RenderState, starting_position: glm::Vec2) -> Option<AnimatedEntity>
    {
        //TODO: rectangular for now because otherwise we stretch onto the rectangular base sprite.
        let possum = match AnimatedEntity::new(render_state,50.0,
            
            vec![
                RenderableConfig::new([0,48],[48,48],0,-0.5),
                RenderableConfig::new([48,48],[48,48],0,-0.5),
                RenderableConfig::new([96,48],[48,48],0,-0.5),
                RenderableConfig::new([144,48],[48,48],0,-0.5),
                RenderableConfig::new([192,48],[48,48],0,-0.5),
                RenderableConfig::new([240,48],[48,48],0,-0.5),
                RenderableConfig::new([288,48],[48,48],0,-0.5),
                RenderableConfig::new([336,48],[48,48],0,-0.5),
            ],
            vec![
                RenderableConfig::new([0,0],[48,48],0,-0.5),
                RenderableConfig::new([48,0],[48,48],0,-0.5),
                RenderableConfig::new([96,0],[48,48],0,-0.5),
                RenderableConfig::new([144,0],[48,48],0,-0.5),
                RenderableConfig::new([192,0],[48,48],0,-0.5),
                RenderableConfig::new([240,0],[48,48],0,-0.5),
                RenderableConfig::new([288,0],[48,48],0,-0.5),
                RenderableConfig::new([336,0],[48,48],0,-0.5),
            ],
            false
        )
        {
            Some(p) => p,
            None => { return None; }
        };

        let transform_loc = match possum.get_transform_location()
        {
            Some(t) => t,
            None => {return None; }
        };

        render_state.set_position_with_index(transform_loc, starting_position);
        render_state.set_scale_with_index(transform_loc,glm::vec3(0.1,0.1,1.0));

        Some(possum)
    }

    pub fn update(&mut self, render_state: &mut RenderState, input_state: &InputState, delta_time: f32)
    {
        let movement_direction = input_state.get_movement_direction();

        for p in &mut self.friendly_possums
        {
            Self::update_animated_entity(p,&movement_direction,render_state,delta_time);
        }
    }

    fn update_animated_entity(animated_entity: &mut AnimatedEntity, movement_direction: &glm::Vec2, render_state: &mut RenderState, delta_time: f32)
    {
        //TODO: bug here, or elsewhere, that causes right-facing possums to be drawn wrong every other frame or so.
        //Only affects the 2nd+ poss in the list, never the first
        //Doesn't affect left facing
        //Stops if we comment out update (specifically animation stepping.)
        //Transforms are independent. Vertices appear to be independent. issue is probably in this file or in AnimatedEntity (latter more probable).
        animated_entity.update(delta_time);

        let transform_loc = match animated_entity.get_transform_location()
        {
            Some(t) => t,
            None => {return; }
        };

        let mut position = match render_state.get_position_with_index(transform_loc)
        {
            Some(pos) => pos,
            None => {return; }
        };

        if movement_direction.x > 0.0 && !animated_entity.get_facing_right()
        {
            animated_entity.set_facing(true);
        } else if movement_direction.x < 0.0 && animated_entity.get_facing_right()
        {
            animated_entity.set_facing(false);
        }

        //"Gravity"
        let mut gravity_affected = false;

        if position.y > 100.0
        {
            gravity_affected = true;
            position.y -= (delta_time / 5.0) * 10.0;

            if position.y < 100.0
            {
                position.y = 100.0;
            }
        }

        if !gravity_affected && movement_direction.x == 0.0 && movement_direction.y == 0.0
        {
            animated_entity.set_animating(false);
            return;
        }

        //TODO: is this the cause of my suffering?
        animated_entity.set_animating(true);

        //TODO: arbitrary speed/ distance
        //TODO: ignoring Y movement
        position.x += (delta_time / 5.0) * movement_direction.x;

        render_state.set_position_with_index(transform_loc, position);
    }

    //TODO: for now, just sprites
    pub fn get_background_renderables(&self) -> &Vec<Sprite>
    {
        &self.tiles
    }

    pub fn get_actor_renderables(&self) -> &Vec<AnimatedEntity>
    {
        &self.friendly_possums
    }
}
use crate::graphics::draw_batch::DrawBatch;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::RenderableConfig;
use crate::game::animated_entity::AnimatedEntity;
use crate::graphics::sprite::Sprite;
use glm::scale;
use rand::Rng;
use crate::util::logging::log;

pub struct GameState
{
    friendly_possums: Vec<AnimatedEntity>,
    tiles: Vec<Sprite>,
    tile_count_x: u32,
    tile_count_y: u32,
    floor_y: u32
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            friendly_possums: Vec::new(),
            tiles: Vec::new(),
            tile_count_x: 10,
            tile_count_y: 10,
            floor_y: 100
        }
    }

    pub fn set_world_size(&mut self, render_state: &mut RenderState)
    {
        let world_size_x = render_state.get_world_size_x();
        let world_size_y = render_state.get_world_size_y();

        let x_placement_offset = world_size_x / self.tile_count_x;
        let y_placement_offset = world_size_y / self.tile_count_y;

        //Since each position is the center of a tile, we offset the initial placement by a tile half width
        let mut next_x_placement =  x_placement_offset as f32 / 2.0;
        let mut next_y_placement = y_placement_offset as f32 / 2.0;

        //Scale of 1.0 fills the screen (see sprite.rs default vertices) as long as we have a square viewport.
        //We want a scale that will make the tile match the tile size
        //TODO: possibly address this issue with scaling later to make scale make more sense.

        let scale_x = 1.0 / self.tile_count_x as f32;
        let scale_y = 1.0 / self.tile_count_y as f32;

        //Tiles start at the bottom left and grow right -> up
        let mut index = 0;
        let z = 2.0; //For tiles
        for tile in &self.tiles
        {
            render_state.set_position(tile, glm::vec3(next_x_placement as f32,next_y_placement as f32, z));
            render_state.set_scale(tile, glm::vec3(scale_x,scale_y,1.0));

            next_x_placement += x_placement_offset as f32;

            if index != 0 && index % self.tile_count_x == 0
            {
                next_y_placement += y_placement_offset as f32;
                next_x_placement = x_placement_offset as f32 / 2.0;
            }

            index += 1;
        }

        for possum in &self.friendly_possums
        {
            let transform_loc = match possum.get_transform_location()
            {
                Some(t) => t,
                None => {continue; }
            };

            render_state.set_scale_with_index(transform_loc, glm::vec3(scale_x,scale_y,1.0));
        }

        self.floor_y = y_placement_offset;

    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {

       let mut rng = rand::thread_rng();

       let mut z = 2.0;

       let world_size_x = render_state.get_world_size_x();

       for i in 0..5
       {
            //TODO: hardcoded
            let y = 200;
            let x = rng.gen_range(0..world_size_x); 

           z -= 0.1;

           let poss = match Self::add_possum(render_state,glm::vec3(x as f32,y as f32,z as f32))
           {
                   Some(p) => p,
                   None => { return; }
           };

           self.friendly_possums.push(poss);
       }

       self.init_tile_grid(render_state);
       self.set_world_size(render_state);
    }

    pub fn init_tile_grid(&mut self, render_state: &mut RenderState)
    {
        //Generate a random tile grid
        let mut rng = rand::thread_rng();

        let use_sprites  = vec![
            RenderableConfig::new([0,0],[100,100],1), //ground
            RenderableConfig::new([100,0],[100,100],1), //background
            RenderableConfig::new([200,0],[100,100],1), //background
            RenderableConfig::new([300,0],[100,100],1), //background
        ];

        for i in 0..(self.tile_count_y * self.tile_count_x) +1
        {
            let mut used_sprite_index = 0;

            if i > self.tile_count_x
            {
                used_sprite_index = rng.gen_range(1..4);
            }

            let tile = match render_state.request_new_renderable::<Sprite>(use_sprites.get(used_sprite_index).unwrap())
            {
                Some(s) => s,
                None => { return; }
            };

            self.tiles.push(tile);
        }

    }

    pub fn add_possum(render_state: &mut RenderState, starting_position: glm::Vec3) -> Option<AnimatedEntity>
    {
        let mut rng = rand::thread_rng();

        let facing = rng.gen_range(0..2) > 0;

        //TODO: rectangular for now because otherwise we stretch onto the rectangular base sprite.
        let possum = match AnimatedEntity::new(render_state,50.0,
            
            vec![
                RenderableConfig::new([0,48],[48,48],0),
                RenderableConfig::new([48,48],[48,48],0),
                RenderableConfig::new([96,48],[48,48],0),
                RenderableConfig::new([144,48],[48,48],0),
                RenderableConfig::new([192,48],[48,48],0),
                RenderableConfig::new([240,48],[48,48],0),
                RenderableConfig::new([288,48],[48,48],0),
                RenderableConfig::new([336,48],[48,48],0),
            ],
            vec![
                RenderableConfig::new([0,0],[48,48],0),
                RenderableConfig::new([48,0],[48,48],0),
                RenderableConfig::new([96,0],[48,48],0),
                RenderableConfig::new([144,0],[48,48],0),
                RenderableConfig::new([192,0],[48,48],0),
                RenderableConfig::new([240,0],[48,48],0),
                RenderableConfig::new([288,0],[48,48],0),
                RenderableConfig::new([336,0],[48,48],0),
            ],
            facing
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

        Some(possum)
    }

    pub fn update(&mut self, render_state: &mut RenderState, input_state: &mut InputState, delta_time: f32)
    {
        let mut rng = rand::thread_rng();

        let x_bound = render_state.get_world_size_x();

        let mut index = 0;
        for p in &mut self.friendly_possums
        {
            let mut movement_direction = glm::vec2(0.0,0.0);
            let mut jumping = false;
            
            //Rudimentary AI
            if index > 0
            {
                let t = match p.get_transform_location()
                {
                    Some(t) => t,
                    None => {continue;}
                };

                let pos = match render_state.get_position_with_index(t)
                {
                    Some(p) => p,
                    None => { continue; }
                };

                if pos.x > x_bound as f32 && p.get_facing_right()
                {
                    p.set_facing(false);
                } else if pos.x < 0.0 && !p.get_facing_right() 
                {
                    p.set_facing(true);
                }

                if p.get_facing_right() 
                {
                    movement_direction = glm::vec2(1.0,0.0);
                } else
                {
                    movement_direction = glm::vec2(-1.0,0.0);
                }
            } else 
            {
                movement_direction = input_state.get_movement_direction();     

                let mut clicked = false;

                while input_state.has_next_click()
                {
                    let click = match input_state.get_next_click()
                    {

                        Some(c) => c,
                        None => { continue; }
                    };

                    clicked = true;
                }
                
                if clicked
                {
                    movement_direction += glm::vec2(0.0,1.0);
                }
            }

            Self::update_animated_entity(p,&movement_direction,render_state,delta_time, self.floor_y);
            index = index + 1;
        }
    }

    pub fn get_renderable_batch(&mut self) -> DrawBatch
    {
        let mut batch = DrawBatch::new();

        for i in &self.tiles
        {
           batch.add_sprite(i);
        }

        for p in &self.friendly_possums
        {
            let r = match p.get_renderable()
            {
                Some(re) => re,
                None => {continue; }
            };

           batch.add_sprite(r);
        }

        batch
    }

    fn update_animated_entity(animated_entity: &mut AnimatedEntity, movement_direction: &glm::Vec2, render_state: &mut RenderState, delta_time: f32, floor_y: u32)
    {
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

        //Vertical movement
        if movement_direction.y > 0.0
        {
            position.y += (delta_time / 5.0) * 200.0;
        }

        //"Gravity"
        let mut gravity_affected = false;

        if position.y > floor_y as f32
        {
            gravity_affected = true;
            position.y -= (delta_time / 5.0) * 10.0;

            if position.y < floor_y as f32
            {
                position.y = floor_y as f32;
            }
        }

        if !gravity_affected && movement_direction.x == 0.0 && movement_direction.y == 0.0
        {
            animated_entity.set_animating(false);
            return;
        }

        animated_entity.set_animating(true);

        //TODO: arbitrary speed/ distance
        //TODO: ignoring Y movement
        position.x += (delta_time / 5.0) * movement_direction.x;

        render_state.set_position_with_index(transform_loc, position);
    }
}
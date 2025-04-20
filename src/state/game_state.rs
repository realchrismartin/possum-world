use crate::graphics::draw_batch::DrawBatch;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::{Renderable,RenderableConfig};
use crate::game::animated_entity::AnimatedEntity;
use crate::graphics::sprite::Sprite;
use rand::Rng;
use crate::util::logging::log;

pub struct GameState
{
    friendly_possums: Vec<AnimatedEntity>,
    tiles: Vec<Sprite>,
    texts: Vec<Sprite>,
    floor_y: f32
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            friendly_possums: Vec::new(),
            tiles: Vec::new(),
            texts: Vec::new(),
            floor_y: 200.0
        }
    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {
        self.friendly_possums.clear();
        self.tiles.clear();
        self.texts.clear();

        //When we resize, we need to clear all of the existing sprite buffers in the render state
        //TODO: update so that we don't have to do this
        render_state.clear_buffer::<Sprite>();
        render_state.clear_transform_buffer();

        let world_size_x = render_state.get_world_size_x();
        let world_size_y = render_state.get_world_size_y();
        //The default sprite size for a tile is 100 x 100
        //Determine how many tiles we need to cover the canvas
        let tile_count_x =  world_size_x / 100;
        let tile_count_y = world_size_y / 100;

        log(format!("Resize requires tiles: {}x{}",tile_count_x,tile_count_y).as_str());

        let x_placement_offset = 100 as f32;
        let y_placement_offset = 100 as f32; 

        log(format!("Using placement offset {}x{}",x_placement_offset,y_placement_offset).as_str());

        //Since each position is the center of a tile, we offset the initial placement by a tile half width
        let mut next_x_placement =  x_placement_offset / 2.0;
        let mut next_y_placement = y_placement_offset / 2.0;

        //Generate tile grid
        let use_sprites  = vec![
            RenderableConfig::new([2,2],[100,100],1), //ground
            RenderableConfig::new([105,2],[100,100],1), //background
            RenderableConfig::new([207,2],[100,100],1), //underground
        ];

        for i in 0..(tile_count_y * tile_count_x) +1
        {
            let mut used_sprite_index = 2; //start with ground

            if i > (tile_count_x * 2)
            {
                //Start using sky once we've created two layers of ground
                used_sprite_index = 1;
            }
            else if i > tile_count_x
            {
                used_sprite_index = 0; //Use top layer
            } 

            let tile = match render_state.request_new_renderable::<Sprite>(use_sprites.get(used_sprite_index).unwrap())
            {
                Some(s) => s,
                None => { return; }
            };

            self.tiles.push(tile);
        }

        //Tiles start at the bottom left and grow right -> up
        let mut index = 0;
        let z = 2.0; //For tiles
        for tile in &self.tiles
        {
            render_state.set_position(tile.get_uid(), glm::vec3(next_x_placement as f32,next_y_placement as f32, z));

            next_x_placement += x_placement_offset as f32;

            if index != 0 && index % tile_count_x == 0
            {
                next_y_placement += y_placement_offset as f32;
                next_x_placement = x_placement_offset as f32 / 2.0;
            }

            index += 1;
        }

        let logo = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([309,2],[368,31],1))
        {
            Some(s) => s,
            None => { return; }
        };

        //NB: 50.0 is from the extra 100 we add as padding to the canvas in index js
        //This gives us roughly the center of the canvas - won't be exact because the 100 is used for overflow (variably)
        let logo_pos = glm::vec3((world_size_x as f32 / 2.0) - 50.0, (world_size_y as f32 / 1.2) + 50.0, 1.9);
        let logo_scale = glm::vec3(1.0,1.0,1.0);

        log(format!("logo pos: {} x {} ",logo_pos.x,logo_pos.y).as_str());

        render_state.set_scale(logo.get_uid(), logo_scale);
        render_state.set_position(logo.get_uid(), logo_pos);
        self.texts.push(logo);

       //Possums
       let mut rng = rand::thread_rng();
       let mut z = 2.0;

        for _i in 0..30
        {
                //TODO: hardcoded
                let y = 300;
                let x = rng.gen_range(0..world_size_x); 

            z -= 0.1;

            let poss = match Self::add_possum(render_state,glm::vec3(x as f32,y as f32,z as f32))
            {
                    Some(p) => p,
                    None => { return; }
            };

            self.friendly_possums.push(poss);
        }

        let mut first = true;
        for possum in &self.friendly_possums
        {
            let s = match possum.get_renderable()
            {
                Some(t) => t,
                None => {continue; }
            };

            if first
            {
                //Barry is larger than the other posses
                render_state.set_scale(s.get_uid(), glm::vec3(3.0,3.0,1.0));
            } else
            {
                render_state.set_scale(s.get_uid(), glm::vec3(2.0,2.0,1.0));
            }

            first = false;
        }
    }

    pub fn add_possum(render_state: &mut RenderState, starting_position: glm::Vec3) -> Option<AnimatedEntity>
    {
        let mut rng = rand::thread_rng();

        let facing = rng.gen_range(0..2) > 0;

        let possum = match AnimatedEntity::new(render_state,50.0,
            
            &vec![
                RenderableConfig::new([2,81],[58,18],0),
                RenderableConfig::new([62,81],[58,18],0),
                RenderableConfig::new([122,81],[58,18],0),
                RenderableConfig::new([182,81],[58,18],0),
                RenderableConfig::new([242,81],[58,18],0),
                RenderableConfig::new([302,81],[58,18],0),
                RenderableConfig::new([362,81],[58,18],0),
                RenderableConfig::new([422,81],[58,18],0),
            ],
            &vec![
                RenderableConfig::new([2,21],[58,18],0),
                RenderableConfig::new([62,21],[58,18],0),
                RenderableConfig::new([122,21],[58,18],0),
                RenderableConfig::new([182,21],[58,18],0),
                RenderableConfig::new([242,21],[58,18],0),
                RenderableConfig::new([302,21],[58,18],0),
                RenderableConfig::new([362,21],[58,18],0),
                RenderableConfig::new([422,21],[58,18],0),
            ],
            facing
        )
        {
            Some(p) => p,
            None => { return None; }
        };

        let s = match possum.get_renderable()
        {
            Some(t) => t,
            None => {return None; }
        };

        render_state.set_position(s.get_uid(),starting_position);

        Some(possum)
    }

    pub fn update(&mut self, render_state: &mut RenderState, input_state: &mut InputState, delta_time: f32)
    {
        let x_bound = render_state.get_world_size_x();

        let mut index = 0;
        for p in &mut self.friendly_possums
        {
            let mut movement_direction = glm::vec2(0.0,0.0);
            
            //Rudimentary AI
            if index > 0
            {
                let s = match p.get_renderable()
                {
                    Some(s) => s,
                    None => {continue;}
                };

                let pos = match render_state.get_position(s.get_uid())
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
                //Set the move direction based on where the mouse is
                if input_state.get_current_mouse_location().is_active()
                {
                    let x = render_state.get_world_size_x();                    

                    if input_state.get_current_mouse_location().get_x_coordinate() > (x/2) as i32
                    {
                        movement_direction = glm::vec2(1.0,0.0);
                    } else {
                        movement_direction = glm::vec2(-1.0,0.0);
                    }
                }

                //TODO: remove this
                /*
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
                 */
            }

            Self::update_animated_entity(p,&movement_direction,render_state,delta_time,self.floor_y);
            index = index + 1;
        }
    }

    pub fn get_renderable_sprite_batch(&mut self) -> DrawBatch<Sprite>
    {
        let mut batch = DrawBatch::<Sprite>::new();

        for i in &self.tiles
        {
           batch.add(i);
        }

        for i in &self.texts
        {
           batch.add(i);
        }

        for p in &self.friendly_possums
        {
            let i = match p.get_renderable()
            {
                Some(re) => re,
                None => {continue; }
            };

           batch.add(i);
        }

        batch
    }

    fn update_animated_entity(animated_entity: &mut AnimatedEntity, movement_direction: &glm::Vec2, render_state: &mut RenderState, delta_time: f32, floor_y: f32)
    {
        animated_entity.update(delta_time);

        let uid = match animated_entity.get_renderable_uid_clone()
        {
            Some(t) => t,
            None => {return; }
        };

        let mut position = match render_state.get_position(&uid)
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

        let size = match animated_entity.get_scaled_size(render_state) 
        {
            Some(s) => s,
            None => {return;}
        };

        //"Gravity"
        let mut gravity_affected = false;

        let adjusted_floor_y = floor_y + (size.y / 2.0);

        if position.y > adjusted_floor_y
        {
            gravity_affected = true;
            position.y -= (delta_time / 5.0) * 10.0;

            if position.y < adjusted_floor_y
            {
                position.y = adjusted_floor_y;
            }
        }

        if (!gravity_affected) && movement_direction.x == 0.0 && movement_direction.y == 0.0
        {
            animated_entity.set_animating(false);
        } else
        {
            animated_entity.set_animating(true);
        }

        //TODO: arbitrary speed/ distance
        //TODO: ignoring Y movement
        position.x += (delta_time / 5.0) * movement_direction.x;

        render_state.set_position(&uid,position);
    }
}
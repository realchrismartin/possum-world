use crate::state::render_state::RenderState;

use crate::graphics::renderable::RenderableConfig;
use crate::game::animated_entity::AnimatedEntity;
use crate::graphics::sprite::Sprite;
use crate::util::logging::log;
use rand::Rng;

pub struct GameState
{
    player_possums: Vec<AnimatedEntity>,
    npc_possums: Vec<AnimatedEntity>,
    tiles: Vec<u32>,
    texts: Vec<u32>,
    next_possum_z: f32,
    player_movement_direction: glm::Vec2
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            player_possums: Vec::new(),
            npc_possums: Vec::new(),
            tiles: Vec::new(),
            texts: Vec::new(),
            next_possum_z: 2.0,
            player_movement_direction: glm::vec2(0.0,0.0)
        }
    }

    pub fn get_player_movement_direction(&self) -> &glm::Vec2
    {
        &self.player_movement_direction
    }

    pub fn set_player_movement_direction(&mut self, direction: &glm::Vec2)
    {
        self.player_movement_direction = *direction;
    }

    pub fn get_tiles(&self) -> &Vec<u32>
    {
        &self.tiles
    }

    pub fn get_texts(&self) -> &Vec<u32>
    {
        &self.texts
    }

    pub fn get_player_possums(&self) -> &Vec<AnimatedEntity>
    {
        &self.player_possums
    }

    pub fn get_npc_possums(&self) -> &Vec<AnimatedEntity>
    {
        &self.npc_possums
    }

    pub fn get_mutable_player_possums(&mut self) -> &mut Vec<AnimatedEntity>
    {
        &mut self.player_possums
    }

    pub fn get_mutable_npc_possums(&mut self) -> &mut Vec<AnimatedEntity>
    {
        &mut self.npc_possums
    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {
        self.player_possums.clear();
        self.npc_possums.clear();
        self.tiles.clear();
        self.texts.clear();

        self.next_possum_z = 1.9;

        self.generate_tile_grid(render_state);
        self.generate_logo(render_state);
        self.generate_player_possums(render_state);
        self.generate_npc_possums(render_state);
    }

    fn generate_tile_grid(&mut self, render_state: &mut RenderState)
    {
        //The default sprite size for a tile is 100 x 100
        //Determine how many tiles we need to cover the canvas
        let world_size_x = render_state.get_canvas_size_x();
        let world_size_y = render_state.get_canvas_size_y();
        let tile_count_x = world_size_x / 100;
        let tile_count_y = world_size_y / 100;

        let x_placement_offset = 100 as f32;
        let y_placement_offset = 100 as f32; 

        //Since each position is the center of a tile, we offset the initial placement by a tile half width
        let mut next_x_placement =  x_placement_offset / 2.0;
        let mut next_y_placement = y_placement_offset / 2.0;

        //Generate tile grid
        let use_sprites  = vec![
            RenderableConfig::new([2,2],[100,100],1), //ground
            RenderableConfig::new([105,2],[100,100],1), //background
            RenderableConfig::new([207,2],[100,100],1), //underground
        ];

        //Tiles start at the bottom left and grow right -> up
        let mut index = 0;
        let z = 2.0; //For tiles

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

            let tile_uid = match render_state.request_new_renderable::<Sprite>(use_sprites.get(used_sprite_index).unwrap())
            {
                Some(s) => s,
                None => { return; }
            };

            self.tiles.push(tile_uid);

            render_state.set_position(&tile_uid, glm::vec3(next_x_placement as f32,next_y_placement as f32, z));

            next_x_placement += x_placement_offset as f32;

            if index != 0 && index % tile_count_x == 0
            {
                next_y_placement += y_placement_offset as f32;
                next_x_placement = x_placement_offset as f32 / 2.0;
            }

            index += 1;
        }
    }

    fn generate_logo(&mut self, render_state: &mut RenderState)
    {
        let logo = match render_state.request_new_renderable::<Sprite>(&RenderableConfig::new([309,2],[368,31],1))
        {
            Some(s) => s,
            None => { return; }
        };

        //NB: 50.0 is from the extra 100 we add as padding to the canvas in index js
        //This gives us roughly the center of the canvas - won't be exact because the 100 is used for overflow (variably)
        let logo_pos = glm::vec3((render_state.get_canvas_size_x() as f32 / 2.0) - 50.0, (render_state.get_canvas_size_y() as f32 / 1.2) + 50.0, 1.9);
        let logo_scale = glm::vec3(1.0,1.0,1.0);

        render_state.set_scale(&logo, logo_scale);
        render_state.set_position(&logo, logo_pos);
        self.texts.push(logo);
    }

    fn generate_player_possums(&mut self, render_state: &mut RenderState)
    {
        let poss = match Self::add_possum(render_state,true,self.next_possum_z)
        {
                Some(p) => p,
                None => { return; }
        };

        self.player_possums.push(poss);
        self.next_possum_z -= 0.01;
    }

    fn generate_npc_possums(&mut self, render_state: &mut RenderState)
    {
        let mut rng = rand::thread_rng();

        for _index in 0..rng.gen_range(4..50)
        {
            let poss = match Self::add_possum(render_state,false,self.next_possum_z)
            {
                    Some(p) => p,
                    None => { return; }
            };

            self.npc_possums.push(poss);
            self.next_possum_z -= 0.01;
        }
    }

    fn add_possum(render_state: &mut RenderState, isPlayer: bool, z: f32) -> Option<AnimatedEntity>
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

        let uid = match possum.get_renderable_uid()
        {
            Some(t) => t,
            None => {return None; }
        };
        
        let scale = rng.gen_range(1.0..4.0);
        let x = rng.gen_range(200..render_state.get_canvas_size_x() - 100) as f32; 
        let y = 600.0; //Hardcoded

        render_state.set_position(uid,glm::vec3(x,y,z));

        if isPlayer
        {
            //Barry is lorger than the other posses
            render_state.set_scale(uid, glm::vec3(5.0,5.0,1.0));
        } else 
        {
            render_state.set_scale(uid, glm::vec3(scale,scale,1.0));
        }

        Some(possum)
    }
}
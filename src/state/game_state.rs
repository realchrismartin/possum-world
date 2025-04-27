use crate::state::render_state::RenderState;

use crate::game::animated_entity::AnimatedEntity;
use crate::graphics::sprite::Sprite;
use crate::graphics::font::Font;
use crate::graphics::text::Text;
use crate::util::logging::log;
use rand::Rng;

pub struct GameState
{
    player_possums: Vec<AnimatedEntity>,
    npc_possums: Vec<AnimatedEntity>,
    tiles: Vec<u32>,
    texts: Vec<u32>,
    base_z: f32,
    z_buffer: f32,
    start_x: f32,
    logo_y: f32,
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
            base_z: 0.0, 
            start_x: 0.0,
            logo_y: 0.0,
            z_buffer: 0.001,
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
        
        self.start_x = render_state.get_canvas_size_x() as f32 / 2.0;
        self.logo_y = 400.0;

        self.generate_tile_grid(render_state);
        self.generate_logo(render_state);
        self.generate_player_possums(render_state);
        self.generate_npc_possums(render_state);
    }

    fn generate_tile_grid(&mut self, render_state: &mut RenderState)
    {
        let use_sprites  = vec![
            Sprite::new([2,2],[100,100],1), //ground
            Sprite::new([105,2],[100,100],1), //background
            Sprite::new([207,2],[100,100],1), //underground
        ];

        let z = self.base_z;

        let underground = match render_state.request_new_renderable::<Sprite>(use_sprites.get(2).unwrap())
        {
            Some(s) => s,
            None => { return; }
        };

        render_state.set_scale(&underground, glm::vec3(100.0,10.0,1.0));
        render_state.set_position(&underground, glm::vec3(0.0 as f32,-350.0 as f32, z));

        self.tiles.push(underground);

        let ground = match render_state.request_new_renderable::<Sprite>(use_sprites.get(0).unwrap())
        {
            Some(s) => s,
            None => { return; }
        };

        render_state.set_scale(&ground, glm::vec3(100.0,2.0,1.0));
        render_state.set_position(&ground, glm::vec3(0.0 as f32,100.0 as f32, z + self.z_buffer));


        self.tiles.push(ground);

        let sky = match render_state.request_new_renderable::<Sprite>(use_sprites.get(1).unwrap())
        {
            Some(s) => s,
            None => { return; }
        };

        render_state.set_scale(&sky, glm::vec3(100.0,100.0,1.0));
        render_state.set_position(&sky, glm::vec3(0.0 as f32,0.0 as f32, z - self.z_buffer));


        self.tiles.push(sky);
    }

    fn generate_logo(&mut self, render_state: &mut RenderState)
    {
        let logo = match render_state.request_new_renderable::<Text>(&Text::new("Possum World", &Font::Default))
        {
            Some(s) => s,
            None => { return; }
        };

        let logo_pos = glm::vec3(self.start_x, self.logo_y, self.base_z + self.z_buffer * 3.0);

        render_state.set_position(&logo, logo_pos);

        let subtext = match render_state.request_new_renderable::<Text>(&Text::new("Insert 1 coin to continue", &Font::Default))
        {
            Some(s) => s,
            None => { return; }
        };

        let subtext_pos = glm::vec3(self.start_x, self.logo_y - 40.0, self.base_z + self.z_buffer * 3.0);

        render_state.set_position(&subtext, subtext_pos);

        //render_state.set_scale(&logo, glm::vec3(0.8,0.8,0.8));
        self.texts.push(logo);
        self.texts.push(subtext);
    }

    fn generate_player_possums(&mut self, render_state: &mut RenderState)
    {
        let poss = match Self::add_possum(render_state,true,self.logo_y,self.base_z + self.z_buffer)
        {
                Some(p) => p,
                None => { return; }
        };

        let uid = match poss.get_renderable_uid()
        {
            Some(u) => u,
            None => { return; }
        };

        render_state.set_position(&uid, glm::vec3(self.start_x,self.logo_y,self.base_z + self.z_buffer));

        self.player_possums.push(poss);
    }

    fn generate_npc_possums(&mut self, render_state: &mut RenderState)
    {
        let mut rng = rand::thread_rng();
        let mut z = self.base_z + self.z_buffer * 2.0;

        for _index in 0..rng.gen_range(4..50)
        {
            let poss = match Self::add_possum(render_state,false,self.logo_y,z)
            {
                    Some(p) => p,
                    None => { return; }
            };

            self.npc_possums.push(poss);
            z += self.z_buffer;
        }
    }

    fn add_possum(render_state: &mut RenderState, isPlayer: bool, y: f32, z: f32) -> Option<AnimatedEntity>
    {
        let mut rng = rand::thread_rng();

        let facing = rng.gen_range(0..2) > 0;

        let possum = match AnimatedEntity::new(render_state,50.0,
            
            &vec![
                Sprite::new([2,81],[58,18],0),
                Sprite::new([62,81],[58,18],0),
                Sprite::new([122,81],[58,18],0),
                Sprite::new([182,81],[58,18],0),
                Sprite::new([242,81],[58,18],0),
                Sprite::new([302,81],[58,18],0),
                Sprite::new([362,81],[58,18],0),
                Sprite::new([422,81],[58,18],0),
            ],
            &vec![
                Sprite::new([2,21],[58,18],0),
                Sprite::new([62,21],[58,18],0),
                Sprite::new([122,21],[58,18],0),
                Sprite::new([182,21],[58,18],0),
                Sprite::new([242,21],[58,18],0),
                Sprite::new([302,21],[58,18],0),
                Sprite::new([362,21],[58,18],0),
                Sprite::new([422,21],[58,18],0),
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
        
        let scale = rng.gen_range(1.0..2.0);
        let x = rng.gen_range(200..render_state.get_canvas_size_x() - 100) as f32; 
        
        render_state.set_position(uid,glm::vec3(x,y,z));

        if isPlayer
        {
            //Barry is lorger than the other posses
            render_state.set_scale(uid, glm::vec3(4.0,4.0,1.0));
        } else 
        {
            render_state.set_scale(uid, glm::vec3(scale,scale,1.0));
        }

        Some(possum)
    }
}
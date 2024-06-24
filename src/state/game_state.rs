use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::renderable::RenderableConfig;
use crate::game::animated_entity::AnimatedEntity;
use crate::graphics::sprite::Sprite;
use rand::Rng;
use crate::util::logging::log;

pub struct GameState
{
    player: Option<AnimatedEntity>,
    friendly_possums: Vec<AnimatedEntity>,
    tiles: Vec<Sprite>
}

impl GameState
{
    pub fn new() -> Self
    {
        Self
        {
            player: None,
            friendly_possums: Vec::new(),
            tiles: Vec::new()
        }
    }

    pub fn init(&mut self, render_state: &mut RenderState)
    {

       self.player = Some(Self::add_possum(render_state,glm::vec2(500.0,100.0)));

       self.init_tile_grid(render_state);

       let mut rng = rand::thread_rng();

       for i in 0..5
       {
            let range = rng.gen_range(50..950);
            self.friendly_possums.push(Self::add_possum(render_state,glm::vec2(range as f32,500.0)));
       }
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
            RenderableConfig::new([100,0],[100,100],1,0.0), //background// TODO: unused
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

    pub fn add_possum(render_state: &mut RenderState, starting_position: glm::Vec2) -> AnimatedEntity
    {
        //TODO: rectangular for now because otherwise we stretch onto the rectangular base sprite.
        let possum = AnimatedEntity::new(render_state,50.0,
            
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
            true
        );

        render_state.set_position_with_index(possum.get_transform_location(), starting_position);
        render_state.set_scale_with_index(possum.get_transform_location(),glm::vec3(0.1,0.1,1.0));

        possum
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

        let movement_direction = input_state.get_movement_direction();

        Self::update_animated_entity(player,&movement_direction,render_state,delta_time);

        for p in &mut self.friendly_possums
        {
            //TODO: use their own movement direction!
            Self::update_animated_entity(p,&movement_direction,render_state,delta_time);
        }
    }

    fn update_animated_entity(animated_entity: &mut AnimatedEntity, movement_direction: &glm::Vec2, render_state: &mut RenderState, delta_time: f32)
    {
        let mut position = match render_state.get_position_with_index(animated_entity.get_transform_location())
        {
            Some(pos) => pos,
            None => {return; }
        };

        animated_entity.update(delta_time);

        if movement_direction.x > 0.0 && !animated_entity.get_facing_right()
        {
            animated_entity.set_facing(true);
        } else if movement_direction.x < 0.0 && animated_entity.get_facing_right()
        {
            animated_entity.set_facing(false);
        }

        //"Gravity"
        let mut gravity_affected = false;

        if(position.y > 100.0)
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

        animated_entity.set_animating(true);

        //TODO: arbitrary speed/ distance
        //TODO: ignoring Y movement
        position.x += (delta_time / 5.0) * movement_direction.x;

        render_state.set_position_with_index(animated_entity.get_transform_location(), position);
    }

    pub fn get_background_renderables(&self) -> &Vec<Sprite>
    {
        //TODO: for now, just sprites
        &self.tiles
    }

    pub fn get_player_renderables(&self) -> &Vec<Sprite>
    {
        let player = match &self.player
        {
            Some(p) => p,
            None => { return &self.tiles; } //TODO: fix this
        };

        player.get_active_sprite()
    }

    pub fn get_npc_entities(&self) -> &Vec<AnimatedEntity>
    {
        &self.friendly_possums
    }
}
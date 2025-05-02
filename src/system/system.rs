use crate::state::game_state::GameState;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::draw_batch::DrawBatch;
use crate::graphics::renderable::Renderable;
use crate::graphics::sprite::Sprite;
use crate::graphics::text::Text;
use crate::game::animated_entity::AnimatedEntity;
use crate::util::logging::log;
use crate::scene::scene::Scene;
use crate::component::component::Component;
use crate::component::physics_component::PhysicsComponent;

pub fn init_systems(game_state: &mut GameState, render_state: &mut RenderState, scene: &mut Scene)
{
    //get data from game state, initialize stuff in render state for it
    render_state.clear();
    game_state.init(render_state);

    //TODO: remove this placeholder stuff.
    let first = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let second = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let third = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    scene.add_component::<PhysicsComponent>(second);
    scene.add_component::<PhysicsComponent>(third);
    scene.add_component::<Sprite>(third);

    scene.run_on_component::<PhysicsComponent, _>(third, |pc: &PhysicsComponent|
    {
        log(format!("before mod entity 3 has PC position {} {}",pc.get_position().x,pc.get_position().y).as_str());
    });

    scene.apply_to_entities_with_both::<PhysicsComponent,Sprite, _>(|entity_uid: usize, pc: &mut PhysicsComponent, s: &mut Sprite|
    {
        let current_pos = pc.get_position();

        pc.set_position(current_pos.x + s.get_size()[0] as f32, current_pos.y + s.get_size()[1] as f32);

        log(format!("test, entity with both a sprite and a PC is {}",entity_uid).as_str());
    });

    scene.run_on_component::<PhysicsComponent, _>(third, |pc: &PhysicsComponent|
    {
        log(format!("after mod entity 3 has PC position {} {}",pc.get_position().x,pc.get_position().y).as_str());
    });
}

pub fn run_systems(scene: &mut Scene, game_state: &mut GameState, render_state: &mut RenderState, input_state: &mut InputState, delta_time : f32)
{
    run_input_system(scene, input_state, render_state, game_state); //TODO: stop passing render state
    run_physics_system(scene, game_state, render_state, delta_time); //TODO: stop passing render state
    run_ai_system(scene, game_state, render_state, delta_time); //TODO: stop passing render state
    run_animation_system(scene, game_state, delta_time);
    run_camera_update_system(scene, game_state, render_state);

    run_render_system(scene, game_state,render_state);
}

//TODO: maybe later remove game_state from here, pass entity deets from elsewhere
fn run_render_system(scene: &mut Scene, game_state: &GameState, render_state: &mut RenderState)
{   
    render_state.clear_context();
    render_state.submit_camera_uniforms(); 
    render_state.bind_and_update_transform_buffer_data();

    let mut sprite_batch = DrawBatch::<Sprite>::new();
    let mut text_batch = DrawBatch::<Text>::new();

    scene.apply_to_entities_with::<Sprite, _>(|entity_uid: usize, sprite: &mut Sprite|
    {
    //    sprite_batch.add(entity_uid);
    });

    /*
    scene.apply_to_entities_with::<Text, _>(|entity_uid: usize, sprite: &mut Sprite|
    {
        text_batch.add(entity_uid);
    });

    scene.apply_to_entities_with::<Animation, _>(|entity_uid: usize, sprite: &mut Animation|
    {
        let uid = match animation.get_renderable_uid()
        {
            Some(u) => {u},
            None => {continue;}
        };

        sprite_batch.add(entity_uid);
    });
    */

    for uid in game_state.get_tiles()
    {
        sprite_batch.add(uid);
    }

    for uid in game_state.get_texts()
    {
        text_batch.add(uid);
    }

    for possum in game_state.get_player_possums()
    {
        let uid = match possum.get_renderable_uid()
        {
            Some(u) => {u},
            None => {continue;}
        };

        sprite_batch.add(uid);
    }

    for possum in game_state.get_npc_possums()
    {
        let uid = match possum.get_renderable_uid()
        {
            Some(u) => {u},
            None => {continue;}
        };

        sprite_batch.add(uid);
    }

    //Render any entities that want to be drawn
    render_state.draw(&sprite_batch);
    render_state.draw(&text_batch);
}

//TODO: apply input to whatever entities want it
//TODO: stop passing all of these state datas
fn run_input_system(scene: &mut Scene, input_state: &InputState, render_state: &RenderState, game_state: &mut GameState)
{
    
    let mut velocity = glm::vec2(0.0,0.0);

    if input_state.get_current_mouse_location().is_active()
    {
        if input_state.get_current_mouse_location().get_x_coordinate() > (render_state.get_canvas_size_x()/2) as i32
        {
            game_state.set_player_movement_direction(&glm::vec2(1.0,0.0)); //TODO
            velocity.x = 1.0;
        } else {
            game_state.set_player_movement_direction(&glm::vec2(-1.0,0.0)); //TODO
            velocity.x = -1.0;
        }
    }

    scene.apply_to_entities_with::<PhysicsComponent, _>(|entity_uid: usize, pc: &mut PhysicsComponent|
    {
       //TODO
       pc.set_velocity(velocity.x,velocity.y);
    });

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
        //TODO
    }
    */
}

fn run_physics_system(scene: &mut Scene, game_state: &mut GameState, render_state: &mut RenderState, delta_time: f32)
{
    //TODO: apply physics to whatever entities want it
    //Update their physics components

    let floor_y = 200.0;

    scene.apply_to_entities_with::<PhysicsComponent, _>(|entity_uid: usize, pc: &mut PhysicsComponent|
    {
            //TODO: ground is hardcoded to be at 200

        //let adjusted_floor_y = floor_y + (size.y / 2.0);
        let adjusted_floor_y = 200.0;

        let new_position;

        {
            let position = pc.get_position();
            let velocity = pc.get_velocity();

            if position.y > adjusted_floor_y
            {
                /*
                position.y -= (delta_time / 5.0) * 10.0;

                if position.y < adjusted_floor_y
                {
                    position.y = adjusted_floor_y;
                }
                */
            }

            //TODO: arbitrary speed/ distance
            new_position = glm::vec2(position.x + (delta_time / 5.0) * velocity.x, position.y + (delta_time / 5.0) * velocity.y);
        }

        pc.set_position(new_position.x,new_position.y);

        log(format!("after input entity has PC position {} {}",pc.get_position().x,pc.get_position().y).as_str());
        log(format!("after input entity has PC velocity {} {}",pc.get_velocity().x,pc.get_velocity().y).as_str());

    });

    //TODO: remove old code when done
    let player_movement_direction = game_state.get_player_movement_direction().clone();

    for possum in game_state.get_mutable_player_possums()
    {
        run_possum_physics(possum,render_state,&player_movement_direction,delta_time);
    }

    for possum in game_state.get_mutable_npc_possums()
    {
        let movement_direction = if possum.get_facing_right() { glm::vec2(1.0,0.0) } else { glm::vec2(-1.0,0.0) }; //TODO: dumb
        run_possum_physics(possum,render_state,&movement_direction,delta_time);
    }
}

fn run_possum_physics(animated_entity: &mut AnimatedEntity, render_state: &mut RenderState, movement_direction: &glm::Vec2, delta_time: f32)
{
    let uid = match animated_entity.get_renderable_uid()
    {
        Some(t) => t.clone(),
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

    //"Gravity"
    let size = match render_state.get_scaled_size(&uid) 
    {
        Some(s) => s,
        None => {return;}
    };

    //TODO: ground is hardcoded to be at 200
    let floor_y = 200.0;

    let adjusted_floor_y = floor_y + (size.y / 2.0);

    if position.y > adjusted_floor_y
    {
        position.y -= (delta_time / 5.0) * 10.0;

        if position.y < adjusted_floor_y
        {
            position.y = adjusted_floor_y;
        }
    }

    if movement_direction.x != 0.0
    {
        animated_entity.set_animating(true);
    } else
    {
        animated_entity.set_animating(false);
        animated_entity.reset_animation();
    }

    //TODO: arbitrary speed/ distance
    position.x += (delta_time / 5.0) * movement_direction.x;
    position.y += (delta_time / 5.0) * movement_direction.y;

    render_state.set_position(&uid,position);
}

fn run_ai_system(scene: &mut Scene, game_state: &mut GameState, render_state: &RenderState, _delta_time: f32)
{
    //TODO: operate only on AI components.
    //Then, pass the AI data to the physics components

    let x_bound = render_state.get_canvas_size_x();

    for possum in game_state.get_mutable_npc_possums()
    {
        let uid = match possum.get_renderable_uid()
        {
            Some(uid) => uid,
            None => {continue;}
        };

        let pos = match render_state.get_position(uid)
        {
            Some(p) => p,
            None => { continue; }
        };

        if pos.x > x_bound as f32 && possum.get_facing_right()
        {
            possum.set_facing(false);
        } else if pos.x < 0.0 && !possum.get_facing_right() 
        {
            possum.set_facing(true);
        }
    }
}

fn run_animation_system(scene: &mut Scene, game_state: &mut GameState, delta_time: f32)
{
    for possum in game_state.get_mutable_player_possums()
    {
        possum.update(delta_time);
    }

    for possum in game_state.get_mutable_npc_possums()
    {
        possum.update(delta_time);
    }
}

fn run_camera_update_system(scene: &mut Scene, game_state: &GameState, render_state: &mut RenderState)
{
    //NB: sets to the first player position

    for possum in game_state.get_player_possums()
    {
        let renderable_uid = match possum.get_renderable_uid()
        {
            Some(u) => u,
            None => {continue;}
        };

        let position = match render_state.get_position(renderable_uid)
        {
            Some(p) => p,
            None => {continue;}
        };

        let cam_pos = glm::vec3(position.x,position.y, position.z);

        render_state.set_camera_world_position(&cam_pos);
        break;
    }
}
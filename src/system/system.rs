use crate::state::game_state::GameState;
use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::draw_batch::DrawBatch;
use crate::graphics::renderable::Renderable;
use crate::graphics::sprite::Sprite;
use crate::graphics::text::Text;
use crate::graphics::animation::Animation;
use crate::game::animated_entity::AnimatedEntity;
use crate::util::logging::log;
use crate::scene::scene::Scene;
use crate::component::physics_component::PhysicsComponent;
use crate::component::component::Component;

pub fn resize_system(scene: &mut Scene, render_state: &mut RenderState)
{
    //Runs when the viewport is resized.
    //TODO: not wired
}

//Runs at game start
pub fn init_scene(scene: &mut Scene)
{
    //TODO: clear scene perhap?

    //The First POSS
    let first = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    scene.add_component::<Sprite>(first, Sprite::new([2,81],[58,18],0));

    //TODO: update renderer to immediately set initial pos from renderable

    //Create grounds 
    //Create grass
    //Create bg
    //Create posses
}

pub fn init_render_data_from_scene(scene: &mut Scene, render_state: &mut RenderState)
{
    //Register our renderrables with the renderstate and get their ids set
    scene.apply_to_entities_with::<Sprite, _>(|entity_uid: usize, component: &mut Sprite|
    {
        render_state.request_new_renderable::<Sprite>(component);
    });

    scene.apply_to_entities_with::<Text, _>(|entity_uid: usize, component: &mut Text|
    {
        render_state.request_new_renderable::<Text>(component);
    });

    scene.apply_to_entities_with::<Animation, _>(|entity_uid: usize, component: &mut Animation|
    {
        //TODO: request multiple sprites and update animation with all renderable uids
    });
}

//Runs every game tick. Updates all of the components, then renders all renderables that get batched.
pub fn run_systems(scene: &mut Scene, game_state: &mut GameState, render_state: &mut RenderState, input_state: &mut InputState, delta_time : f32)
{
    run_cursor_position_system(input_state,render_state);
    run_input_system(scene, input_state, render_state, game_state); //TODO: only take scene and input state as args?
    run_physics_system(scene, game_state, render_state, delta_time); //TODO: stop passing render state
    run_ai_system(scene, game_state, render_state, delta_time); //TODO: stop passing render state
    run_animation_system(scene, game_state, delta_time);
    run_camera_update_system(scene, game_state, render_state);

    run_render_system(scene, game_state,render_state); //TODO: stop passing game state
}

//TODO: stop passing mutable ref to scene here (need to allow const iteration first)
fn load_batch_for_renderable_type<T: Renderable + Component>(scene: &mut Scene, batch: &mut DrawBatch<T>)
{
    scene.apply_to_entities_with::<T, _>(|entity_uid: usize, renderable: &mut T|
    {
        batch.add(&renderable.get_renderable_uid());
    });
}

//TODO: can we somehow generify this?
fn load_batch_for_animation(scene: &mut Scene, batch: &mut DrawBatch<Sprite>)
{
    scene.apply_to_entities_with::<Animation, _>(|entity_uid: usize, animation: &mut Animation|
    {
        batch.add(&animation.get_renderable_uid());
    });
}

fn run_render_system(scene: &mut Scene, game_state: &GameState, render_state: &mut RenderState)
{   
    render_state.clear_context();
    render_state.submit_camera_uniforms(); 
    render_state.bind_and_update_transform_buffer_data();

    //TODO: can we determine which batches we want to make by iterating over a static array of types?
    let mut sprite_batch = DrawBatch::<Sprite>::new();
    let mut text_batch = DrawBatch::<Text>::new();

    load_batch_for_renderable_type::<Sprite>(scene,&mut sprite_batch);
    load_batch_for_renderable_type::<Text>(scene, &mut text_batch);
    load_batch_for_animation(scene,&mut sprite_batch);

    //TODO: old code - remove when deprecated!
    /*
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
    */

    //Render any entities that want to be drawn
    render_state.draw(&sprite_batch);
    render_state.draw(&text_batch);
}

fn run_cursor_position_system(input_state: &InputState, render_state: &RenderState)
{
    //TODO: move the calculation of the coordinates here as x and y ratio (0 ... 1) and store that data on the input state
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
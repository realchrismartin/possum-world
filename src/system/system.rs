use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::draw_batch::DrawBatch;
use crate::graphics::renderable::Renderable;
use crate::graphics::animation::{AnimationState,Animation};
use crate::graphics::sprite::Sprite;
use crate::graphics::text::Text;
use crate::scene::scene::Scene;
use crate::graphics::font::Font;
use crate::component::physics_body::PhysicsBody;
use crate::component::player_input::PlayerInput;
use crate::component::ai::{AIState, AI};
use crate::component::component::Component;
use crate::networking::server_connection::{ServerConnection,OutboundMessage,InboundMessage, MessageType};
use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;
use crate::util::logging::log;

//Runs at game start
pub fn init_scene(scene: &mut Scene)
{
    scene.clear();

    //The First POSS
    let player = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let bg = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let ground = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let underground = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let logo = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let logo_subtitle = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    //BG, ground
    scene.add_component::<Sprite>(bg, Sprite::new_with_position([105,2],[100,100],1, glm::vec2(0.0,0.0),-2.0, glm::vec2(1000.0,1000.0)));
    scene.add_component::<Sprite>(ground, Sprite::new_with_position([2,2],[100,100],1, glm::vec2(0.0,-75.0),-1.0, glm::vec2(100.0,1.0)));
    scene.add_component::<Sprite>(underground, Sprite::new_with_position([207,2],[100,100],1, glm::vec2(0.0,-550.0), -1.5, glm::vec2(100.0,10.0)));

    let mut rng = rand::thread_rng();

    //Grasses        
    let mut z = -1.5;

    for _index in 0..rng.gen_range(50..120)
    {
        let grass = match scene.add_entity()
        {
            Some(e) => e,
            None => {return;}
        };

        z += 0.002;

        scene.add_component::<Sprite>(grass, Sprite::new_with_position([309,2],[62,46],1, glm::vec2(rng.gen_range(-1000.0..1000.0),-15.0),z, glm::vec2(1.0,1.0)));
    }

    //NPC Posses
    z = -0.75;

    for _index in 0..rng.gen_range(4..10)
    {
        let poss = match scene.add_entity()
        {
            Some(e) => e,
            None => {continue;}
        };

        z += 0.002;

        scene.add_component::<PhysicsBody>(poss, PhysicsBody::new_with_position(glm::vec2(rng.gen_range(-250.0..250.0),-25.0)));
        scene.add_component::<AI>(poss, AI::new());
        scene.add_component::<Animation::<Sprite>>(poss,Animation::<Sprite>::new(
            HashMap::from([
                (AnimationState::FacingRight, vec![
                    Sprite::new([2,21],[58,18],0)
                ]),
                (AnimationState::FacingLeft, vec![
                    Sprite::new([2,81],[58,18],0),
                ]),
                (AnimationState::WalkingLeft, vec![
                    Sprite::new([2,81],[58,18],0),
                    Sprite::new([62,81],[58,18],0),
                    Sprite::new([122,81],[58,18],0),
                    Sprite::new([182,81],[58,18],0),
                    Sprite::new([242,81],[58,18],0),
                    Sprite::new([302,81],[58,18],0),
                    Sprite::new([362,81],[58,18],0),
                    Sprite::new([422,81],[58,18],0),
                ]),
                (AnimationState::WalkingRight, vec![
                    Sprite::new([2,21],[58,18],0),
                    Sprite::new([62,21],[58,18],0),
                    Sprite::new([122,21],[58,18],0),
                    Sprite::new([182,21],[58,18],0),
                    Sprite::new([242,21],[58,18],0),
                    Sprite::new([302,21],[58,18],0),
                    Sprite::new([362,21],[58,18],0),
                    Sprite::new([422,21],[58,18],0),
                ]),
            ]),
            AnimationState::FacingRight,
            50.0,
            glm::vec2(0.0,0.0),
            z,
            glm::vec2(2.0,2.0)
        ));
    }

    //Player Possum ("Barry")
    scene.add_component::<PlayerInput>(player, PlayerInput::new());
    scene.add_component::<PhysicsBody>(player, PhysicsBody::new());
    scene.add_component::<Animation::<Sprite>>(player,Animation::<Sprite>::new(
        HashMap::from([
            (AnimationState::FacingRight, vec![
                Sprite::new([2,21],[58,18],0)
            ]),
            (AnimationState::FacingLeft, vec![
                Sprite::new([2,81],[58,18],0),
            ]),
            (AnimationState::WalkingLeft, vec![
                Sprite::new([2,81],[58,18],0),
                Sprite::new([62,81],[58,18],0),
                Sprite::new([122,81],[58,18],0),
                Sprite::new([182,81],[58,18],0),
                Sprite::new([242,81],[58,18],0),
                Sprite::new([302,81],[58,18],0),
                Sprite::new([362,81],[58,18],0),
                Sprite::new([422,81],[58,18],0),
            ]),
            (AnimationState::WalkingRight, vec![
                Sprite::new([2,21],[58,18],0),
                Sprite::new([62,21],[58,18],0),
                Sprite::new([122,21],[58,18],0),
                Sprite::new([182,21],[58,18],0),
                Sprite::new([242,21],[58,18],0),
                Sprite::new([302,21],[58,18],0),
                Sprite::new([362,21],[58,18],0),
                Sprite::new([422,21],[58,18],0),
            ]),
        ]),
        AnimationState::FacingRight,
        50.0,
        glm::vec2(0.0,0.0),
        0.0001,
        glm::vec2(5.0,5.0)
    ));

    //Logo Text
    scene.add_component::<Text>(logo, Text::new_with_position("Possum World", &Font::Default, glm::vec2(0.0,350.0), 0.002, glm::vec2(2.0,2.0)));
    scene.add_component::<Text>(logo_subtitle, Text::new_with_position("insert 1 coin to continue", &Font::Default, glm::vec2(0.0,200.0), 0.002, glm::vec2(2.0,2.0)));
}

//Register our renderables and types which own renderables with the renderstate and get their ids set
pub fn init_render_data_from_scene(scene: &mut Scene, render_state: &mut RenderState)
{
    render_state.clear();
    render_state.clear_buffer::<Sprite>();
    render_state.clear_buffer::<Text>();

    scene.apply_to_entities_with::<Sprite, _>(|component: &mut Sprite|
    {
        render_state.request_new_renderable::<Sprite>(component);
    });

    scene.apply_to_entities_with::<Text, _>(|component: &mut Text|
    {
        render_state.request_new_renderable::<Text>(component);
    });

    scene.apply_to_entities_with::<Animation<Sprite>, _>(|component: &mut Animation<Sprite>|
    {
        let mut first_renderable_uid : Option<u32> = None;
        component.apply_to_renderables(|renderable: &mut Sprite|
        {
            if first_renderable_uid.is_some()
            {
                render_state.request_new_renderable_with_existing_transform::<Sprite>(renderable, first_renderable_uid.unwrap());
            } else
            {
                render_state.request_new_renderable::<Sprite>(renderable);

                first_renderable_uid = Some(renderable.get_renderable_uid());
            }
        });

        if first_renderable_uid.is_none()
        {
            return;
        }

        let uid = &first_renderable_uid.unwrap();

        //Since we don't provide default position/scale/etc for animations, set it once on the shared transform here
        //NB: all frames have the same data, including scale.
        render_state.set_position(uid, component.get_starting_world_position());
        render_state.set_z(uid, component.get_starting_z());
        render_state.set_scale(uid, component.get_starting_scale());
    });

    scene.apply_to_entities_with::<Animation<Text>, _>(|component: &mut Animation<Text>|
    {
        let mut first_renderable_uid : Option<u32> = None;
        component.apply_to_renderables(|renderable: &mut Text|
        {
            if first_renderable_uid.is_some()
            {
                render_state.request_new_renderable_with_existing_transform::<Text>(renderable, first_renderable_uid.unwrap());
            } else
            {
                render_state.request_new_renderable::<Text>(renderable);
                first_renderable_uid = Some(renderable.get_renderable_uid());
            }
        });
    });

    //TODO: scale visuals to fit physics body sizes? how to handle sizes? (text size is incorrect in the ctor, animations have multiple sizes)
}

//Runs whenever we want to remove an entity.
//Any time we add a component, we need to add a call here, otherwise the buffers will never remove components for these entities...
//TODO: this could be nicer..
pub fn remove_entity(scene: &mut Scene, render_state: &mut RenderState, entity_uid: usize)
{
    scene.apply_to_entity::<Sprite, _>(entity_uid,|renderable: &mut Sprite|
    {
        render_state.free_renderable(renderable);
    });

    scene.apply_to_entity::<Text, _>(entity_uid,|renderable: &mut Text|
    {
        render_state.free_renderable(renderable);
    });

    scene.apply_to_entity::<Animation<Sprite>, _>(entity_uid, |component: &mut Animation<Sprite>|
    {
        component.apply_to_renderables(|renderable: &mut Sprite|
        {
            render_state.free_renderable(renderable);
        });
    });

    scene.apply_to_entity::<Animation<Text>, _>(entity_uid, |component: &mut Animation<Text>|
    {
        component.apply_to_renderables(|renderable: &mut Text|
        {
            render_state.free_renderable(renderable);
        });
    });

    scene.remove_component::<Sprite>(entity_uid);
    scene.remove_component::<Text>(entity_uid);
    scene.remove_component::<Animation::<Sprite>>(entity_uid);
    scene.remove_component::<Animation::<Text>>(entity_uid);
    scene.remove_component::<PhysicsBody>(entity_uid);
    scene.remove_component::<PlayerInput>(entity_uid);
}

//Runs every game tick. Updates all of the components, then renders all renderables that get batched.
pub fn run_systems(scene: &mut Scene, render_state: &mut RenderState, input_state: &mut InputState, server_connection: &mut ServerConnection, delta_time : f32)
{
    run_networking_system(scene, server_connection, render_state, delta_time); //TODO: remove render state 
    run_input_system(scene, input_state); 
    run_physics_system(scene, delta_time);
    run_ai_system(scene, delta_time);
    run_animation_system(scene, delta_time);
    run_update_render_from_physics_system(scene, render_state);
    run_camera_update_system(scene, render_state);
    run_render_system(scene, render_state); 
}

//TODO: stop passing mutable ref to scene here (need to allow const iteration first)
fn load_batch_for_renderable_type<T: Renderable + Component>(scene: &mut Scene, batch: &mut DrawBatch<T>)
{
    scene.apply_to_entities_with::<T, _>(|renderable: &mut T|
    {
        batch.add(&renderable.get_renderable_uid());
    });

    scene.apply_to_entities_with::<Animation<T>, _>(|animation: &mut Animation<T>|
    {
        match animation.get_renderable_uid()
        {
            Some(u) => { batch.add(&u); }
            None => {}
        }
    });
}

fn run_networking_system(scene: &mut Scene, server_connection: &mut ServerConnection, render_state: &mut RenderState, delta_time: f32)
{
    scene.apply_to_entities_with_both::<PlayerInput, PhysicsBody, _>(|_player_input: &mut PlayerInput, physics_body: &mut PhysicsBody|
    {
        server_connection.send_message_if_ready(&OutboundMessage::new(physics_body.get_position().x,physics_body.get_position().y), delta_time);
    });

    let mut rng = rand::thread_rng();

    server_connection.receive_inbound_messages(&mut |message : &InboundMessage|
    {
        let mut entity_uid : Option<usize> = None;

        {
            match scene.get_entity_for_peer(message.uuid())
            {
                Some(e) => {entity_uid = Some(*e);}
                None => {}
            };
        }

        match message.message_type()
        {
            MessageType::Update => {},
            MessageType::Departure => {

                log(&format!("{} is departing...",message.uuid()));

                if !entity_uid.is_none()
                {
                    //Tell the scene to remove the entity from the map
                    scene.remove_entity_for_peer(message.uuid());

                    //Also remove components. We have to do this separately for now because generics..
                    remove_entity(scene,render_state, entity_uid.unwrap());
                }

                return;
            }
        };

        match entity_uid
        {
            Some(euid) => {
                scene.apply_to_entity::<PhysicsBody, _>(euid, |physics_body : &mut PhysicsBody| 
                {
                    //Make peers walk to their current position
                    let x = physics_body.get_position().x - *message.x();

                    if x < 10.0 && x > -10.0
                    {
                        physics_body.set_velocity(0.0,0.0);
                    } else if x > 10.0
                    {
                        physics_body.set_velocity(-1.0,0.0);
                    } else
                    {
                        physics_body.set_velocity(1.0,0.0);
                    }
                });
            },
            None => {

                let peer_entity = match scene.add_entity_for_peer(message.uuid())
                {
                    Some(e) => e,
                    None => {return;}
                };

                log(&format!("{} has arrived!",message.uuid()));

                scene.add_component::<PhysicsBody>(peer_entity, PhysicsBody::new());
                scene.add_component::<Animation::<Sprite>>(peer_entity,Animation::<Sprite>::new(
                    HashMap::from([
                        (AnimationState::FacingRight, vec![
                            Sprite::new([2,21],[58,18],0)
                        ]),
                        (AnimationState::FacingLeft, vec![
                            Sprite::new([2,81],[58,18],0),
                        ]),
                        (AnimationState::WalkingLeft, vec![
                            Sprite::new([2,81],[58,18],0),
                            Sprite::new([62,81],[58,18],0),
                            Sprite::new([122,81],[58,18],0),
                            Sprite::new([182,81],[58,18],0),
                            Sprite::new([242,81],[58,18],0),
                            Sprite::new([302,81],[58,18],0),
                            Sprite::new([362,81],[58,18],0),
                            Sprite::new([422,81],[58,18],0),
                        ]),
                        (AnimationState::WalkingRight, vec![
                            Sprite::new([2,21],[58,18],0),
                            Sprite::new([62,21],[58,18],0),
                            Sprite::new([122,21],[58,18],0),
                            Sprite::new([182,21],[58,18],0),
                            Sprite::new([242,21],[58,18],0),
                            Sprite::new([302,21],[58,18],0),
                            Sprite::new([362,21],[58,18],0),
                            Sprite::new([422,21],[58,18],0),
                        ]),
                    ]),
                    AnimationState::FacingRight,
                    50.0,
                    glm::vec2(0.0,0.0),
                    -0.75,
                    glm::vec2(5.0,5.0)
                ));

                scene.apply_to_entity::<Animation<Sprite>, _>(peer_entity, |component: &mut Animation<Sprite>|
                {
                    let mut first_renderable_uid : Option<u32> = None;
                    component.apply_to_renderables(|renderable: &mut Sprite|
                    {
                        if first_renderable_uid.is_some()
                        {
                            render_state.request_new_renderable_with_existing_transform::<Sprite>(renderable, first_renderable_uid.unwrap());
                        } else
                        {
                            render_state.request_new_renderable::<Sprite>(renderable);
            
                            first_renderable_uid = Some(renderable.get_renderable_uid());
                        }
                    });
            
                    if first_renderable_uid.is_none()
                    {
                        return;
                    }
            
                    let uid = &first_renderable_uid.unwrap();
            
                    //Since we don't provide default position/scale/etc for animations, set it once on the shared transform here
                    //NB: all frames have the same data, including scale.
                    render_state.set_position(uid, component.get_starting_world_position());
                    render_state.set_z(uid, component.get_starting_z());
                    render_state.set_scale(uid, component.get_starting_scale());
                });

                let names = vec!["Lumpy Nick", "Lumpy Regan", "Lumpy J", "Lumpy Mike", "Pointy Nick", "Pointy Regan", "Pointy J", "Pointy Mike"];
                let name = names[rng.gen_range(0..names.len())];
                scene.add_component::<Text>(peer_entity, Text::new_with_position(name, &Font::Default, glm::vec2(0.0,150.0), 0.002, glm::vec2(1.0,1.0)));

                scene.apply_to_entity::<Text, _>(peer_entity,|component: &mut Text|
                {
                    render_state.request_new_renderable::<Text>(component);
                });
            }
        }
    });
}

fn run_render_system(scene: &mut Scene, render_state: &mut RenderState)
{   
    render_state.clear_context();
    render_state.submit_camera_uniforms(); 
    render_state.bind_and_update_transform_buffer_data();

    let mut sprite_batch = DrawBatch::<Sprite>::new();
    let mut text_batch = DrawBatch::<Text>::new();

    load_batch_for_renderable_type(scene,&mut sprite_batch); //Sprites
    load_batch_for_renderable_type(scene, &mut text_batch); //Texts

    //Render any entities that want to be drawn
    render_state.draw(&mut sprite_batch);
    render_state.draw(&mut text_batch);
}

fn run_input_system(scene: &mut Scene, input_state: &mut InputState)
{
    let mut velocity = glm::vec2(0.0,0.0);

    if input_state.get_current_mouse_location().is_active()
    {
        if *input_state.get_current_mouse_location().get_canvas_ratio_x() > 0.5
        {
            velocity.x = 1.0;
        } else {
            velocity.x = -1.0;
        }
    }

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

    scene.apply_to_entities_with_both::<PhysicsBody, PlayerInput, _>(|physics_body: &mut PhysicsBody, _player_input: &mut PlayerInput|
    {
       physics_body.set_velocity(velocity.x,velocity.y);
    });

}

fn run_physics_system(scene: &mut Scene,  delta_time: f32)
{
    scene.apply_to_entities_with::<PhysicsBody, _>(|component: &mut PhysicsBody|
    {
        //Apply drag

        //position.y -= (delta_time / 5.0) * 10.0;
        
        //Change position based on velocity
        //TODO: mass later?
        let new_position = glm::vec2(component.get_position().x + (delta_time / 5.0) * component.get_velocity().x, component.get_position().y + (delta_time / 5.0) * component.get_velocity().y);
        component.set_position(new_position.x,new_position.y);
    });
}

fn run_ai_system(scene: &mut Scene, delta_time: f32)
{
    scene.apply_to_entities_with::<AI, _>(|component: &mut AI|
    {
        component.update(delta_time);
    });

    scene.apply_to_entities_with_both::<AI, PhysicsBody, _>(|ai: &mut AI, physics_body: &mut PhysicsBody|
    {
        match ai.get_state()
        {
            AIState::Idling => { physics_body.set_velocity(0.0,0.0); }
            AIState::WalkingLeft => { physics_body.set_velocity(-1.0,0.0); }
            AIState::WalkingRight => { physics_body.set_velocity(1.0,0.0); }
        };
    });

    //For now:
    //Set the state of the animation based on the velocity direction
    scene.apply_to_entities_with_both::<Animation<Sprite>, PhysicsBody, _>(|animation: &mut Animation<Sprite>, physics_body: &mut PhysicsBody|
    {
        if physics_body.get_velocity().x == 0.0
        {
            match animation.get_animation_state()
            {
                AnimationState::FacingRight => {},
                AnimationState::FacingLeft => {},
                AnimationState::WalkingLeft => { animation.set_animation_state(AnimationState::FacingLeft)},
                AnimationState::WalkingRight => { animation.set_animation_state(AnimationState::FacingRight)},
            };

            animation.set_animating(false);
        } else if physics_body.get_velocity().x > 0.0
        {
            animation.set_animating(true);
            animation.set_animation_state(AnimationState::WalkingRight);
        } else
        {
            animation.set_animating(true);
            animation.set_animation_state(AnimationState::WalkingLeft);
        }
    });
}

fn run_animation_system(scene: &mut Scene, delta_time: f32)
{
    scene.apply_to_entities_with::<Animation<Sprite>, _>(|component: &mut Animation<Sprite>|
    {
        component.update(delta_time);
    });

    scene.apply_to_entities_with::<Animation<Text>, _>(|component: &mut Animation<Text>|
    {
        component.update(delta_time);
    });
}

fn run_camera_update_system(scene: &mut Scene, render_state: &mut RenderState)
{
    scene.apply_to_entities_with_both::<PlayerInput, PhysicsBody, _>(|_player_input: &mut PlayerInput, physics_body: &mut PhysicsBody|
    {
        render_state.set_camera_world_position(physics_body.get_position());
    });
}

fn run_update_render_from_physics_system(scene: &mut Scene, render_state: &mut RenderState)
{
    scene.apply_to_entities_with_both::<PhysicsBody, Sprite, _>(|physics_body: &mut PhysicsBody, renderable: &mut Sprite|
    {
        render_state.set_position(&renderable.get_renderable_uid(), &physics_body.get_position());
    });

    scene.apply_to_entities_with_both::<PhysicsBody, Text, _>(|physics_body: &mut PhysicsBody, renderable: &mut Text|
    {
        //TODO: hacked to allow for nametags for now
        //render_state.set_position(&renderable.get_renderable_uid(), &physics_body.get_position());
        let body_pos = physics_body.get_position();
        let offset_pos = glm::vec2(body_pos.x, body_pos.y - 75.0);
        render_state.set_position(&renderable.get_renderable_uid(),&offset_pos);
    });

    scene.apply_to_entities_with_both::<PhysicsBody, Animation<Sprite>, _>(|physics_body: &mut PhysicsBody, animation: &mut Animation<Sprite>|
    {
        match animation.get_renderable_uid()
        {
            Some(u) => { 
                render_state.set_position(&u, &physics_body.get_position());
            }
            None => {}
        }
    });

    scene.apply_to_entities_with_both::<PhysicsBody, Animation<Text>, _>(|physics_body: &mut PhysicsBody, animation: &mut Animation<Text>|
    {
        match animation.get_renderable_uid()
        {
            Some(u) => {
                render_state.set_position(&u, &physics_body.get_position());
            }
            None => {}
        }
    });
}

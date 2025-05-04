use crate::state::input_state::InputState;
use crate::state::render_state::RenderState;

use crate::graphics::draw_batch::DrawBatch;
use crate::graphics::renderable::Renderable;
use crate::graphics::animation::{AnimationState,Animation};
use crate::graphics::sprite::Sprite;
use crate::graphics::text::Text;
use crate::util::logging::log;
use crate::scene::scene::Scene;
use crate::graphics::font::Font;
use crate::component::physics_component::PhysicsComponent;
use crate::component::component::Component;
use std::collections::HashMap;

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

    let bg = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    let logo = match scene.add_entity()
    {
        Some(e) => e,
        None => {return;}
    };

    scene.add_component::<Sprite>(bg, Sprite::new_with_position([105,2],[100,100],1, glm::vec3(0.0,0.0,-2.0)));

    scene.add_component::<PhysicsComponent>(first, PhysicsComponent::new());
    scene.add_component::<Animation::<Sprite>>(first,Animation::<Sprite>::new(
        HashMap::from([
            (AnimationState::Default, vec![
                Sprite::new([2,81],[58,18],0),
                Sprite::new([62,81],[58,18],0),
                Sprite::new([122,81],[58,18],0),
                Sprite::new([182,81],[58,18],0),
                Sprite::new([242,81],[58,18],0),
                Sprite::new([302,81],[58,18],0),
                Sprite::new([362,81],[58,18],0),
                Sprite::new([422,81],[58,18],0),
            ])
        ]),
        50.0
    ));


    scene.add_component::<Text>(logo, Text::new("Possum World", &Font::Default));

    //Create grounds 
    //Create grass
    //Create bg
    //Create posses
}

//Register our renderables and types which own renderables with the renderstate and get their ids set
pub fn init_render_data_from_scene(scene: &mut Scene, render_state: &mut RenderState)
{
    scene.apply_to_entities_with::<Sprite, _>(|entity_uid: usize, component: &mut Sprite|
    {
        render_state.request_new_renderable::<Sprite>(component);
    });

    scene.apply_to_entities_with::<Text, _>(|entity_uid: usize, component: &mut Text|
    {
        render_state.request_new_renderable::<Text>(component);
    });

    scene.apply_to_entities_with::<Animation<Sprite>, _>(|entity_uid: usize, component: &mut Animation<Sprite>|
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
    });

    scene.apply_to_entities_with::<Animation<Text>, _>(|entity_uid: usize, component: &mut Animation<Text>|
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
}

//Runs every game tick. Updates all of the components, then renders all renderables that get batched.
pub fn run_systems(scene: &mut Scene, render_state: &mut RenderState, input_state: &mut InputState, delta_time : f32)
{
    run_cursor_position_system(input_state,render_state);
    run_input_system(scene, input_state); 
    run_physics_system(scene, delta_time);
    run_ai_system(scene, delta_time);
    run_animation_system(scene, delta_time);
    run_update_render_from_physics_system(scene, render_state);
    run_camera_update_system(scene, render_state);

    run_render_system(scene, render_state); //TODO: stop passing game state
}

//TODO: stop passing mutable ref to scene here (need to allow const iteration first)
fn load_batch_for_renderable_type<T: Renderable + Component>(scene: &mut Scene, batch: &mut DrawBatch<T>)
{
    scene.apply_to_entities_with::<T, _>(|entity_uid: usize, renderable: &mut T|
    {
        batch.add(&renderable.get_renderable_uid());
    });

    scene.apply_to_entities_with::<Animation<T>, _>(|entity_uid: usize, animation: &mut Animation<T>|
    {
        batch.add(&animation.get_renderable_uid());
    });
}

fn run_render_system(scene: &mut Scene, render_state: &mut RenderState)
{   
    render_state.clear_context();
    render_state.submit_camera_uniforms(); 
    render_state.bind_and_update_transform_buffer_data();

    //TODO: can we determine which batches we want to make by iterating over a static array of types?
    let mut sprite_batch = DrawBatch::<Sprite>::new();
    let mut text_batch = DrawBatch::<Text>::new();

    load_batch_for_renderable_type(scene,&mut sprite_batch); //Sprites
    load_batch_for_renderable_type(scene, &mut text_batch); //Texts

    //Render any entities that want to be drawn
    render_state.draw(&sprite_batch);
    render_state.draw(&text_batch);
}

fn run_cursor_position_system(input_state: &InputState, render_state: &RenderState)
{
    //TODO: move the calculation of the coordinates here as x and y ratio (0 ... 1) and store that data on the input state
    //if input_state.get_current_mouse_location().get_x_coordinate() > (render_state.get_canvas_size_x()/2) as i32
}

//TODO: apply input to whatever entities want it
//TODO: stop passing all of these state datas
fn run_input_system(scene: &mut Scene, input_state: &InputState)
{
    
    let mut velocity = glm::vec2(0.0,0.0);

    if input_state.get_current_mouse_location().is_active()
    {
        velocity.x = -1.0; //TODO
        /*
        if input_state.get_current_mouse_location().get_x_coordinate() > (render_state.get_canvas_size_x()/2) as i32
        {
            velocity.x = 1.0;
        } else {
            velocity.x = -1.0;
        }
        */
    }

    scene.apply_to_entities_with::<PhysicsComponent, _>(|entity_uid: usize, pc: &mut PhysicsComponent|
    {
       //TODO: only affect player
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

fn run_physics_system(scene: &mut Scene,  delta_time: f32)
{
    scene.apply_to_entities_with::<PhysicsComponent, _>(|entity_uid: usize, component: &mut PhysicsComponent|
    {
        //position.y -= (delta_time / 5.0) * 10.0;
        let new_position = glm::vec2(component.get_position().x + (delta_time / 5.0) * component.get_velocity().x, component.get_position().y + (delta_time / 5.0) * component.get_velocity().y);
        component.set_position(new_position.x,new_position.y);
    });
}

fn run_ai_system(scene: &mut Scene, _delta_time: f32)
{
    //TODO: operate only on AI components.
    //Then, pass the AI data to the physics components
}

fn run_animation_system(scene: &mut Scene, delta_time: f32)
{
    //TODO: generify over renderable types?

    scene.apply_to_entities_with::<Animation<Sprite>, _>(|entity_uid: usize, component: &mut Animation<Sprite>|
    {
        component.update(delta_time);
    });

    scene.apply_to_entities_with::<Animation<Text>, _>(|entity_uid: usize, component: &mut Animation<Text>|
    {
        component.update(delta_time);
    });
}

fn run_camera_update_system(scene: &mut Scene, render_state: &mut RenderState)
{
    //TODO: set to the player position instead of whatever has a PC
    scene.apply_to_entities_with::<PhysicsComponent, _>(|entity_uid: usize, component: &mut PhysicsComponent|
    {
        //TODO
        render_state.set_camera_world_position(component.get_position());
    });
}

fn run_update_render_from_physics_system(scene: &mut Scene, render_state: &mut RenderState)
{
    //TODO: generify over renderable types?
    scene.apply_to_entities_with_both::<PhysicsComponent, Animation<Sprite>, _>(|entity_uid: usize, physics_component: &mut PhysicsComponent, animation: &mut Animation<Sprite>|
    {
        //TODO: z hardcoded
        let z = 0.0;

        let vec_3_pos = glm::vec3(physics_component.get_position().x, physics_component.get_position().y, z);

        render_state.set_position(&animation.get_renderable_uid(), vec_3_pos);
    });
    //TODO: add other renderables to have their positions updated here
}
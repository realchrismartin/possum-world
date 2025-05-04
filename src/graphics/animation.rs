use crate::component::component::Component;
use crate::graphics::renderable::Renderable;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum AnimationState 
{
    Default,
    FacingLeft,
    FacingRight,
    WalkingRight,
    WalkingLeft
}

impl AnimationState 
{
}

#[derive(Clone)]
pub struct Animation<T: Renderable>
{
    renderable_map: HashMap<AnimationState,Vec<T>>,
    current_animation_state: AnimationState,
    current_renderable_index: usize,
    animating: bool,
    time_since_frame_change: f32,
    time_per_frame: f32,
    starting_world_position: glm::Vec2,
    starting_z: f32,
    starting_scale: glm::Vec2,
}

impl<T: Renderable> Component for Animation<T>
{

}

impl<T: Renderable> Animation<T>
{
    pub fn new(
        renderable_map: HashMap<AnimationState,Vec<T>>,
        default_state: AnimationState,
        time_per_frame: f32,
        starting_world_position: glm::Vec2,
        starting_z: f32,
        starting_scale: glm::Vec2,
    ) -> Self
    {
        Self
        {
            renderable_map: renderable_map,
            current_renderable_index: 0,
            current_animation_state: default_state,
            animating: false,
            time_since_frame_change: 0.0,
            time_per_frame: time_per_frame,
            starting_world_position,
            starting_z,
            starting_scale
        }
    }

    pub fn apply_to_renderables<F>(&mut self, mut functor: F)
    where
        F: FnMut(&mut T)
    {
        for (_state_key, mutable_renderable_vec) in self.renderable_map.iter_mut() {

            for renderable in mutable_renderable_vec.iter_mut()
            {
                functor(renderable);
            }
        }
    }

    pub fn get_renderable_uid(&self) -> Option<u32> 
    {
        let renderables_for_state = match self.renderable_map.get(&self.current_animation_state)
        {
            Some(r) => r,
            None => {return None;}
        };

        let renderable = match renderables_for_state.get(self.current_renderable_index)
        {
            Some(u) => u,
            None => {return None;}
        };

        Some(renderable.get_renderable_uid())
    }

    pub fn step_animation(&mut self)
    {
        let renderables_for_state = match self.renderable_map.get(&self.current_animation_state)
        {
            Some(r) => r,
            None => {return;}
        };

        if renderables_for_state.is_empty()
        {
            return;
        }

        self.current_renderable_index = (self.current_renderable_index + 1) % renderables_for_state.len();
    }

    pub fn update(&mut self, delta_time: f32)
    {
        if !self.animating
        {
            return;
        }

        self.time_since_frame_change += delta_time;

        if self.time_since_frame_change >= self.time_per_frame
        {
            self.step_animation();
            self.time_since_frame_change = 0.0;
        }
    }

    pub fn set_animating(&mut self, state: bool)
    {
        self.animating = state;
    }

    pub fn reset_animation(&mut self)
    {
        self.time_since_frame_change = 0.0;
        self.current_renderable_index = 0;
    }

    pub fn set_animation_state(&mut self, state: AnimationState)
    {
        if self.current_animation_state == state
        {
            return;
        }

        self.current_animation_state = state;
        self.reset_animation();
    }

    pub fn get_animation_state(&self) -> &AnimationState
    {
        &&self.current_animation_state
    }

    pub fn get_starting_world_position(&self) -> &glm::Vec2
    {
        &&self.starting_world_position
    }

    pub fn get_starting_z(&self) -> f32
    {
        self.starting_z
    }

    pub fn get_starting_scale(&self) -> &glm::Vec2
    {
        &&self.starting_scale
    }
}

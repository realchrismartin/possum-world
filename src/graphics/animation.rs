use crate::component::component::Component;
use crate::graphics::renderable::Renderable;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum AnimationState 
{
    Default,
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
}

impl<T: Renderable> Component for Animation<T>
{

}

impl<T: Renderable> Animation<T>
{
    pub fn new(
        renderable_map: HashMap<AnimationState,Vec<T>>,
        time_per_frame: f32
    ) -> Self
    {
        Self
        {
            renderable_map: renderable_map,
            current_renderable_index: 0,
            current_animation_state: AnimationState::Default,
            animating: true,
            time_since_frame_change: 0.0,
            time_per_frame: time_per_frame
        }
    }

    pub fn apply_to_renderables<F>(&mut self, mut functor: F)
    where
        F: FnMut(&mut T)
    {
        for (state_key, mutable_renderable_vec) in self.renderable_map.iter_mut() {

            for renderable in mutable_renderable_vec.iter_mut()
            {
                functor(renderable);
            }
        }
    }

    pub fn get_renderable_uid(&self) -> u32
    {
        let renderables_for_state = match self.renderable_map.get(&self.current_animation_state)
        {
            Some(r) => r,
            None => {return 0;}
        };

        let renderable = match renderables_for_state.get(self.current_renderable_index)
        {
            Some(u) => u,
            None => {return 0;}
        };

        renderable.get_renderable_uid()
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
        self.current_animation_state = state;
    }
}

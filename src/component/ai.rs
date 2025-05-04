use crate::component::component::Component;
use rand::Rng;

#[derive(Clone)]
pub enum AIState 
{
    Idling, WalkingLeft, WalkingRight
}

impl AIState {}

#[derive(Clone)]
pub struct AI 
{
    state: AIState,
    time_since_decision : f32,
    time_per_decision: f32
}

impl AI 
{
    pub fn new() -> Self
    {
        Self
        {
            state: AIState::Idling,
            time_since_decision: 0.0,
            time_per_decision: 600.0 //TODO: hardcoded
        }
    }

    pub fn update(&mut self, delta_time: f32)
    {
        self.time_since_decision += delta_time;

        if self.time_since_decision >= self.time_per_decision
        {
            self.decide_new_state();
            self.time_since_decision = 0.0;
        }
    }

    pub fn get_state(&self) -> &AIState
    {
        &&self.state
    }

    fn decide_new_state(&mut self)
    {
        let mut rng = rand::thread_rng();
        let outcome = rng.gen_range(0..2);

        if outcome == 0
        {
            self.state = AIState::WalkingLeft;
        } else if outcome == 1
        {
            self.state = AIState::WalkingRight;
        } else
        {
            self.state = AIState::Idling;
        }
    }
}

impl Component for AI 
{
}
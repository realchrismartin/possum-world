use crate::component::component::Component;

#[derive(Clone)]
pub struct PlayerInput 
{
}

impl PlayerInput 
{
    pub fn new() -> Self
    {
        Self
        {
        }

    }
}

impl Component for PlayerInput
{
}
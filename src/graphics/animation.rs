use crate::component::component::Component;

#[derive(Copy)]
#[derive(Clone)]
pub struct Animation {
    current_renderable_uid: u32
}

impl Animation 
{
    pub fn new() -> Self
    {
        Self
        {
            current_renderable_uid: 0 //TODO
        }
    }

    pub fn get_renderable_uid(&self) -> u32
    {
        self.current_renderable_uid
    }

    //TODO: set which UIDs this uses
}

impl Component for Animation 
{
}
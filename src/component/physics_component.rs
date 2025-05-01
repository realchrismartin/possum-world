use crate::component::component::Component;

#[derive(Copy)]
#[derive(Clone)]
pub struct PhysicsComponent
{
    position: glm::Vec2
}

impl PhysicsComponent
{
    fn get_position(&self) -> &glm::Vec2
    {
        &&self.position
    }

    fn set_position(&mut self, position : &glm::Vec2)
    {
        self.position.x = position.x;
        self.position.y = position.y;
    }
}

impl Component for PhysicsComponent
{
    fn new() -> Self
    {
        Self
        {
            position: glm::vec2(0.0,0.0)
        }
    }
}
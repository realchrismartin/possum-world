use crate::component::component::Component;

#[derive(Copy)]
#[derive(Clone)]
pub struct PhysicsComponent
{
    position: glm::Vec2
}

impl PhysicsComponent
{
    pub fn get_position(&self) -> &glm::Vec2
    {
        &&self.position
    }

    pub fn set_position(&mut self, x: f32, y: f32)
    {
        self.position.x = x;
        self.position.y = y;
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
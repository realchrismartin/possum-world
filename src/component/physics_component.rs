use crate::component::component::Component;

#[derive(Clone)]
pub struct PhysicsBody
{
    position: glm::Vec2,
    velocity: glm::Vec2
}

impl PhysicsBody
{
    pub fn new() -> Self
    {
        Self
        {
            position: glm::vec2(0.0,0.0),
            velocity: glm::vec2(0.0,0.0)
        }

    }
    pub fn get_position(&self) -> &glm::Vec2
    {
        &&self.position
    }

    pub fn set_position(&mut self, x: f32, y: f32)
    {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn get_velocity(&self) -> &glm::Vec2
    {
        &&self.velocity
    }

    pub fn set_velocity(&mut self, x: f32, y: f32)
    {
        self.velocity.x = x;
        self.velocity.y = y;
    }
}

impl Component for PhysicsBody
{
}
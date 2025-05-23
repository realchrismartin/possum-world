pub struct Transform
{
    translation: glm::Vec3,
    z_rotation: f32,
    scale: glm::Vec3,
}

//Holds the raw components and the dirty flag for a specific transform in the transform buffer.
impl Transform
{
    pub fn new() -> Self
    {
        Self
        {
            translation: glm::vec3(0.0,0.0,0.0),
            z_rotation: 0.0,
            scale: glm::vec3(1.0,1.0,1.0),
        }
    }

    pub fn reset(&mut self)
    {
        self.translation.x = 0.0;
        self.translation.y = 0.0;
        self.translation.z = 0.0;
        self.z_rotation = 0.0;
        self.scale.x = 1.0;
        self.scale.y = 1.0;
        self.scale.z = 1.0;
    }

    pub fn set_translation(&mut self, translation: &glm::Vec2)
    {
        self.translation.x = translation.x;
        self.translation.y = translation.y;
    }

    pub fn set_z(&mut self, z: f32)
    {
        self.translation.z = z;
    }

    pub fn get_z(&self) -> &f32
    {
        &&self.translation.z
    }

    pub fn set_scale(&mut self, scale: &glm::Vec2)
    {
        self.scale.x = scale.x;
        self.scale.y = scale.y;
    }

    pub fn set_rotation(&mut self, rotation: f32)
    {
        self.z_rotation = rotation;
    }

    pub fn calculate(&self) -> glm::Mat4
    {
        let mut matrix = glm::Mat4::identity().into();

        matrix = glm::translate(&matrix,&self.translation);
        matrix = glm::rotate(&matrix,self.z_rotation,&glm::vec3(0.0,0.0,1.0)); //For now, only Z rotation
        matrix = glm::scale(&matrix,&self.scale);

        matrix
    }
}

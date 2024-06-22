pub struct Transform
{
    translation: glm::Vec3,
    z_rotation: f32,
    scale: glm::Vec3,
    dirty: bool
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
            dirty: false, //We start out with an identity matrix in the buffer - no need to recalc it immediately.
        }
    }

    pub fn dirty(&self) -> bool
    {
        self.dirty
    }

    pub fn set_clean(&mut self)
    {
        self.dirty = false;
    }

    pub fn set_translation(&mut self, translation: glm::Vec3)
    {
        self.translation = translation;
        self.dirty = true;
    }

    pub fn set_scale(&mut self, scale: glm::Vec3)
    {
        self.scale = scale;
        self.dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: f32)
    {
        self.z_rotation = rotation;
        self.dirty = true;
    }

    pub fn calculate(&mut self) -> glm::Mat4
    {
        let mut matrix = glm::Mat4::identity().into();

        matrix = glm::translate(&matrix,&self.translation);
        matrix = glm::rotate(&matrix,self.z_rotation,&glm::vec3(0.0,0.0,1.0)); //For now, only Z rotation
        matrix = glm::scale(&matrix,&self.scale);

        self.dirty = false;

        matrix
    }
}
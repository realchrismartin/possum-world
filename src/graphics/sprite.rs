pub struct Sprite 
{
    vertices: [f32;12],
    indices: [u32;6]
}

impl Sprite 
{
    pub fn new() -> Self
    {

        //TODO: don't hardcode this data later.
        //TODO: associate this type with its vertex type
        
        Sprite 
        {
            vertices:
            [
                -0.3,0.3,0.0,
                -0.5,-0.5,0.0,
                0.5,-0.5,0.0,
                0.3,0.3,0.0
            ],

            indices: [0,1,2,2,3,0]
        }
    }

    pub fn get_vertices(&self) -> &[f32]
    {
        return &self.vertices;
    }

    pub fn get_indices(&self) -> &[u32]
    {
        return &self.indices;
    }
}
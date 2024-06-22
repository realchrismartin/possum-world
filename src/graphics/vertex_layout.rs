pub struct VertexLayoutElement
{
    pub location: i32,
    pub size: i32
}

impl VertexLayoutElement
{
    pub fn new(location :i32, size: i32) -> Self
    {
        Self
        {
            location,
            size
        }
    }
}

pub struct VertexLayout
{
    ordered_layout_elements: Vec<VertexLayoutElement>
}

impl VertexLayout
{
    pub fn new(ordered_layout_elements: Vec<VertexLayoutElement>) -> Self
    {
        Self
        {
            ordered_layout_elements
        }
    }
    
    pub fn get_elements(&self) -> &Vec<VertexLayoutElement>
    {
        &self.ordered_layout_elements
    }
}
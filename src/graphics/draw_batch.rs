use crate::graphics::renderable::Renderable;
use std::marker::PhantomData;
use std::ops::Range;

pub struct DrawBatch<T>
{
    _phantom: PhantomData<T>, //Hint that we will use the type T later
    ranges: Vec<Range<i32>>
}

impl<T: Renderable> DrawBatch<T>
{
    pub fn new() -> Self
    {
        Self 
        {
            _phantom: PhantomData,
            ranges: Vec::new()
        }
    }

    pub fn add(&mut self, renderable: &dyn Renderable)
    {
        let range = match renderable.get_element_location()
        {
            Some(r) => r,
            None => { return; }
        };

        self.ranges.push(range.clone());
    }

    pub fn get_ranges(&self) -> &Vec<Range<i32>>
    {
        &self.ranges
    }
}
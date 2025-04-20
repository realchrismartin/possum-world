use crate::graphics::renderable::Renderable;
use std::marker::PhantomData;

pub struct DrawBatch<T>
{
    _phantom: PhantomData<T>, //Hint that we will use the type T later
    uids: Vec<u32>
}

impl<T: Renderable> DrawBatch<T>
{
    pub fn new() -> Self
    {
        Self 
        {
            _phantom: PhantomData,
            uids: Vec::new()
        }
    }

    pub fn add(&mut self, uid: &u32)
    {
        self.uids.push(uid.clone());
    }

    pub fn get_uids(&self) -> &Vec<u32>
    {
        &self.uids
    }
}
use crate::component::component::Component;

pub struct ComponentBuffer<T>
{
    components: [Option<T>;1000]
}

impl<T:Component> ComponentBuffer<T>
{
    pub fn new() -> Self
    {
        Self
        {
            components: [None;1000]
        }
    }

    pub fn add(&mut self, index: usize)
    {
        if index >= self.components.len()
        {
            return;
        }

        if self.components[index].is_some()
        {
            return;
        }

        self.components[index] = Some(T::new());
    }

    pub fn remove(&mut self, index: usize)
    {
        if index >= self.components.len()
        {
            return;
        }

        self.components[index] = None;
    }

    pub fn get(&self, index: usize) -> Option<&T>
    {
        if index >= self.components.len()
        {
            return None;
        }

        if self.components[index].is_none()
        {
            return None;
        }

        return self.components[index].as_ref();
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T>
    {
        if index >= self.components.len()
        {
            return None;
        }

        if self.components[index].is_none()
        {
            return None;
        }

        return self.components[index].as_mut();
    }
}


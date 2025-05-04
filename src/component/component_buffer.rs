use crate::component::component::Component;
use std::collections::HashSet;

pub struct ComponentBuffer<T>
{
    components: Vec<Option<T>>,
    entity_set: HashSet<usize>
}

impl<T:Component> ComponentBuffer<T>
{
    pub fn new() -> Self
    {
        Self
        {
            components: vec![None; 1000],
            entity_set: HashSet::with_capacity(1000)
        }
    }

    pub fn add(&mut self, index: usize, component: T)
    {
        if self.entity_set.contains(&index)
        {
            return;
        }

        match self.components.get(index)
        {
            Some(s) => {return;}
            None => {}
        };

        self.components[index] = Some(component);
        self.entity_set.insert(index);
    }

    pub fn remove(&mut self, index: usize)
    {
        if !self.entity_set.contains(&index)
        {
            return;
        }

        match self.components.get(index)
        {
            Some(s) => {return;}
            None => {}
        };

        self.components[index] = None;
        self.entity_set.remove(&index);
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

        self.components[index].as_ref()
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

        self.components[index].as_mut()
    }

    pub fn get_entity_set(&self) -> &HashSet<usize>
    {
        &&self.entity_set
    }
}


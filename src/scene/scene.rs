use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use crate::component::component::Component;
use crate::component::component_buffer::ComponentBuffer;

pub struct Scene
{
    next_entity_uid: usize,
    component_buffer_map: HashMap<TypeId,Box<dyn Any + 'static>>,
}

impl Scene
{
    pub fn new() -> Self
    {
        Self
        {
            next_entity_uid: 0,
            component_buffer_map: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self) -> Option<usize>
    {
        //Can't have more entities than component buffer size
        if self.next_entity_uid >= 99
        {
            return None;
        }

        self.next_entity_uid += 1;
        return Some(self.next_entity_uid);
    }

    pub fn add_component<T: Component>(&mut self, entity_uid: usize)
    {
        //Lazy init the component buffer for this type
        self.init_component_buffer::<T>();

        let mut_buffer = match Self::get_mut_component_buffer::<T>(&mut self.component_buffer_map)
        {
            Some(b) => b,
            None => { return; }
        };

        mut_buffer.add(entity_uid);
    }

    pub fn get_component<T: Component>(&self, entity_uid: usize) -> Option<&T>
    {
        let buffer = match Self::get_component_buffer::<T>(&self.component_buffer_map)
        {
            Some(b) => b,
            None => { return None; }
        };

        buffer.get(entity_uid)
    }

    pub fn get_mut_component<T: Component>(&mut self, entity_uid: usize) -> Option<&mut T>
    {
        let buffer = match Self::get_mut_component_buffer::<T>(&mut self.component_buffer_map)
        {
            Some(b) => b,
            None => { return None; }
        };

        buffer.get_mut(entity_uid)
    }

    fn init_component_buffer<T: Component>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        if self.component_buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return;
        }

        self.component_buffer_map.insert(type_id,Box::new(ComponentBuffer::<T>::new()));
    }

    fn get_component_buffer<T: Component>(buffer_map: & HashMap<TypeId,Box<dyn Any>>) -> Option<&ComponentBuffer<T>>
    {
        let type_id = TypeId::of::<T>();

        if !buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = match buffer_map.get(&type_id)
        {
            Some(boxed_buffer) => boxed_buffer,
            None => {return None;}
        };

        boxed_buffer.downcast_ref::<ComponentBuffer<T>>()
    }

    fn get_mut_component_buffer<T: Component>(buffer_map: &mut HashMap<TypeId,Box<dyn Any>>) -> Option<&mut ComponentBuffer<T>>
    {
        let type_id = TypeId::of::<T>();

        if !buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = match buffer_map.get_mut(&type_id)
        {
            Some(boxed_buffer) => boxed_buffer,
            None => {return None;}
        };

        boxed_buffer.downcast_mut::<ComponentBuffer<T>>()
    }
}
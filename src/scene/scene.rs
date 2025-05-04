use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use std::cell::RefCell;
use core::cell::Ref;
use core::cell::RefMut;
use crate::component::component::Component;
use crate::component::component_buffer::ComponentBuffer;
pub struct Scene
{
    next_entity_uid: usize,
    component_buffer_map: HashMap<TypeId,RefCell<Box<dyn Any + 'static>>>,
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

    pub fn clear(&mut self)
    {
        self.next_entity_uid = 0;
        self.component_buffer_map.clear();
    }

    pub fn apply_to_entities_with_both<T: Component, U: Component, F>(&mut self, mut functor: F)
    where
        F: FnMut(&mut T, &mut U)
    {
        //First, create a list of uids that are all entities which have both component types
        let intersection : Vec<usize>;
        {
            let buffer_a = match Self::get_component_buffer::<T>(&self.component_buffer_map)
            {
                Some(a) => a,
                None => { return; }
            };

            let buffer_b = match Self::get_component_buffer::<U>(&self.component_buffer_map)
            {
                Some(b) => b,
                None => { return; }
            };

            let set_a = buffer_a.get_entity_set();
            let set_b = buffer_b.get_entity_set();

            intersection = set_a.intersection(&set_b).cloned().collect();
        }

        let component_buffer_map = &mut self.component_buffer_map;

        let buffer_a_ref = match component_buffer_map.get(&TypeId::of::<T>()) 
        {
            Some(a) => a,
            None => { return; }
        };

        let buffer_b_ref = match component_buffer_map.get(&TypeId::of::<U>()) 
        {
            Some(b) => b,
            None => { return; }
        };

        let mut mut_borrow_a = buffer_a_ref.borrow_mut();
        let buffer_a = mut_borrow_a.downcast_mut::<ComponentBuffer<T>>().unwrap();

        let mut mut_borrow_b = buffer_b_ref.borrow_mut();
        let buffer_b = mut_borrow_b.downcast_mut::<ComponentBuffer<U>>().unwrap();

        for entity_uid in intersection
        {
            let component_instance_a = match buffer_a.get_mut(entity_uid)
            {
                Some(a) => a,
                None => { continue; }
            };

            let component_instance_b = match buffer_b.get_mut(entity_uid)
            {
                Some(b) => b,
                None => { continue; }
            };

            functor(component_instance_a,component_instance_b);
        }
    }

    pub fn apply_to_entities_with<T: Component, F>(&mut self, mut functor: F)
    where
        F: FnMut(&mut T)
    {
        
        let set: Vec<usize>;

        {
            let buffer = match Self::get_component_buffer::<T>(&self.component_buffer_map)
            {
                Some(a) => a,
                None => { return; }
            };

            set = buffer.get_entity_set().iter().cloned().collect();
        }

        let component_buffer_map = &mut self.component_buffer_map;

        let buffer_ref = match component_buffer_map.get(&TypeId::of::<T>()) 
        {
            Some(a) => a,
            None => { return; }
        };

        let mut mut_borrow = buffer_ref.borrow_mut();
        let buffer = mut_borrow.downcast_mut::<ComponentBuffer<T>>().unwrap();

        for entity_uid in set 
        {
            let component_instance = match buffer.get_mut(entity_uid)
            {
                Some(a) => a,
                None => { continue; }
            };

            functor(component_instance);
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

    pub fn add_component<T: Component>(&mut self, entity_uid: usize, component: T)
    {
        //Lazy init the component buffer for this type
        self.init_component_buffer::<T>();

        let mut mut_buffer = match Self::get_mut_component_buffer::<T>(&mut self.component_buffer_map)
        {
            Some(b) => b,
            None => { return; }
        };

        mut_buffer.add(entity_uid, component);
    }

    fn init_component_buffer<T: Component>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        if self.component_buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return;
        }

        self.component_buffer_map.insert(type_id,RefCell::new(Box::new(ComponentBuffer::<T>::new())));
    }

    fn get_component_buffer<T: Component>(buffer_map: & HashMap<TypeId,RefCell<Box<dyn Any>>>) -> Option<Ref<ComponentBuffer<T>>>
    {
        let type_id = TypeId::of::<T>();

        if !buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = buffer_map.get(&type_id).unwrap(); 

        Some(Ref::map(boxed_buffer.borrow(), |any| {
            any.downcast_ref::<ComponentBuffer<T>>().unwrap()
        }))
    }

    fn get_mut_component_buffer<T: Component>(buffer_map: &mut HashMap<TypeId,RefCell<Box<dyn Any>>>) -> Option<RefMut<ComponentBuffer<T>>>
    {
        let type_id = TypeId::of::<T>();

        if !buffer_map.contains_key(&type_id)
        {
            //nothing to do
            return None;
        }

        let boxed_buffer = buffer_map.get_mut(&type_id).unwrap(); 

        Some(RefMut::map(boxed_buffer.borrow_mut(), |any| {
            any.downcast_mut::<ComponentBuffer<T>>().unwrap()
        }))
    }
}
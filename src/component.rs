use std::{
    collections::HashMap,
    any::{
        TypeId,
        Any
    },
};
  
pub trait Component {}

pub struct ComponentList<T> {
    pub components: Vec<T>,
    pub entity_map: HashMap<usize, usize>
}

impl<T: Component + 'static> ComponentList<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::<T>::new(),
            entity_map: HashMap::new()
        }
    }

    pub fn add_component(&mut self, entity: usize, component: T) {
        let index = self.components.len();
        self.entity_map.insert(entity, index);
        self.components.push(component);
    }

    pub fn get_entity_component(&self, entity: usize) -> Option<&T> {
        let index = self.entity_map.get(&entity);
        match index {
            Some(i) => Some(&self.components[*i]),
            None => None
        }
    }

    pub fn get_entity_component_mut(&mut self, entity: usize) -> Option<&mut T> {
        let index = self.entity_map.get(&entity);
        match index {
            Some(i) => Some(&mut self.components[*i]),
            None => None
        }
    }
}

pub struct ComponentManager {
    pub component_lists: HashMap<TypeId, Box<dyn Any>>
}

impl ComponentManager {  
    pub fn new() -> Self {
        Self {
            component_lists: HashMap::new()
        }
    }

    pub fn register_components<T: 'static>(&mut self) where T : Component {
        self.component_lists.insert(TypeId::of::<T>(), Box::new(ComponentList::<T>::new()));
    }

    pub fn get_components<T: 'static>(&self) -> Option<&ComponentList<T>> where T : Component {
        let boxed_component_lists = self.component_lists.get(&TypeId::of::<T>());
        match boxed_component_lists {
            Some(b) => {
                let component_lists = b.downcast_ref::<ComponentList<T>>();
                match component_lists {
                    Some(c) => Some(c),
                    None => None
                }
            },
            None => None
        }
    }

    pub fn get_components_mut<T: 'static>(&mut self) -> Option<&mut ComponentList<T>> where T : Component {
        let boxed_component_lists = self.component_lists.get_mut(&TypeId::of::<T>());
        match boxed_component_lists {
            Some(b) => {
                let component_lists = b.downcast_mut::<ComponentList<T>>();
                match component_lists {
                    Some(c) => Some(c),
                    None => None
                }
            },
            None => None
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: usize, component: T) where T : Component {
        let component_lists = self.get_components_mut::<T>();
        match component_lists {
            Some(c) => c.add_component(entity, component),
            None => {
                self.register_components::<T>();
                self.add_component::<T>(entity, component);
            }
        }
    }

    pub fn entity_has_component<T: 'static>(&self, entity: usize) -> bool where T : Component {
        let component_lists = self.get_components::<T>();
        match component_lists {
            Some(c) => c.entity_map.contains_key(&entity),
            None => false
        }
    }
}

use std::{
  collections::HashMap,
  any::{
      TypeId,
  }
};
use ggez::{Context, GameResult};

use crate::{
  entity::EntitySystem,
  component::ComponentManager
};

pub trait System {
  fn update(&mut self, ctx: &mut Context, entities: usize, component_manager: &mut ComponentManager);
  fn draw(&self, ctx: &mut Context, entities: usize, component_manager: &ComponentManager) -> GameResult;
}

pub struct SystemManager {
  pub entity_system: EntitySystem,
  pub systems: HashMap<TypeId, Box<dyn System>>
}

#[allow(dead_code)]
impl SystemManager {
  pub fn new() -> Self {
    Self {
      entity_system: EntitySystem::new(),
      systems: HashMap::new(),
    }
  }

  pub fn register_system<T: 'static>(&mut self) where T: Default + System {
    let system = T::default();
    self.systems.insert(TypeId::of::<T>(), Box::new(system));
  }

  pub fn get_system<T: 'static>(&self) -> Option<&Box<dyn System>> where T : System {
    self.systems.get(&TypeId::of::<T>())
  }

  pub fn update(&mut self, ctx: &mut Context, component_manager: &mut ComponentManager) {
    for (_, system) in &mut self.systems {
      system.update(ctx, self.entity_system.entities, component_manager);
    }
  }

  pub fn draw(&self, ctx: &mut Context, component_manager: &ComponentManager) -> GameResult {
    for (_, system) in &self.systems {
      system.draw(ctx, self.entity_system.entities, component_manager)?;
    }

    Ok(())
  }
}

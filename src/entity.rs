pub struct Entity(pub usize);

pub struct EntitySystem {
  pub entities: usize
}

impl EntitySystem {
  pub fn new() -> Self {
    Self {
      entities: 0,
    }
  }

  pub fn create_entity(&mut self) -> Entity {
    let new_entity_id = self.entities;
    self.entities += 1;
    Entity(new_entity_id)
  }
}
use crate::entity::Entity;

#[derive(Debug)]
pub struct Cell {
    entities: Vec<Entity>,
}

impl Cell {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn get_all(&self) -> Vec<&Entity> {
        self.entities.iter().collect()
    }

    pub fn get_all_mut(&mut self) -> Vec<&mut Entity> {
        self.entities.iter_mut().collect()
    }

    pub fn insert(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn get_all_cloned(&self) -> Vec<Entity> {
        self.entities.clone()
    }
}

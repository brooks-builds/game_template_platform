use crate::entity::Entity;

#[derive(Debug)]
pub struct Cell {
    pub width: f32,
    pub height: f32,
    entities: Vec<Entity>,
}

impl Cell {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            entities: vec![],
        }
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

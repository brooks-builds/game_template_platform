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

    pub fn insert(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
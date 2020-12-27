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

    pub fn take_by_id(&mut self, id: u32) -> Option<Entity> {
        if let Some(index) = self.entities.iter().position(|entity| entity.id == id) {
            Some(self.entities.remove(index))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::entity::builder::EntityBuilder;

    use super::*;

    #[test]
    fn ci_test_cell_take_by_id() {
        let mut cell = Cell::new();
        let mut entity_builder = EntityBuilder::new();
        let entity_0 = entity_builder.create_entity().build();
        let entity_1 = entity_builder.create_entity().build();
        cell.insert(entity_0);
        cell.insert(entity_1);
        let entity: Entity = cell.take_by_id(0).unwrap();
        assert_eq!(entity.id, 0);
        assert_eq!(cell.entities.len(), 1);
    }
}

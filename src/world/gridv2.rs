use std::collections::HashMap;

use eyre::eyre;
use eyre::Result;
use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;

use crate::{entity, Entity};

pub struct Grid {
    width: u32,
    height: u32,
    cells: HashMap<(u32, u32), Vec<u32>>,
    unit_width: f32,
    unit_height: f32,
}

impl Grid {
    pub fn new(world_width: f32, world_height: f32, unit_width: f32, unit_height: f32) -> Self {
        let width = (world_width / unit_width) as u32;
        let height = (world_height / unit_height) as u32;
        let mut cells = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                cells.insert((x, y), vec![]);
            }
        }

        Self {
            width,
            height,
            cells,
            unit_width,
            unit_height,
        }
    }

    pub fn insert(&mut self, entity: &Entity) -> Result<()> {
        let coordinates = self.get_coordinates(entity.location.x, entity.location.y);

        let cell = self.cells.get_mut(&coordinates).ok_or_else(|| {
            eyre!(
                "error inserting entity into grid at coordinates: {:?}",
                coordinates
            )
        })?;

        cell.push(entity.id);

        Ok(())
    }

    /// Get the ids of the entities that are within the query rectangle
    pub fn query(&self, query: Rect) -> Vec<&u32> {
        let mut found_ids = vec![];
        let start = self.get_coordinates(query.x, query.y);
        let end = self.get_coordinates(query.x + query.w, query.y + query.h);

        let mut indexes = vec![];
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                indexes.push((x, y));
            }
        }

        indexes.iter().for_each(|index| {
            if let Some(cell) = self.cells.get(index) {
                cell.iter().for_each(|id| found_ids.push(id));
            }
        });

        found_ids
    }

    pub fn update_entity_location(&mut self, old_location: Vector2<f32>, entity: &Entity) {
        let old_coordinates = self.get_coordinates(old_location.x, old_location.y);
        let new_coordinates = self.get_coordinates(entity.location.x, entity.location.y);
        if old_coordinates != new_coordinates {
            // remove the entity id from the old cell
            // add entity id to new cell
        }
    }

    fn get_coordinates(&self, x: f32, y: f32) -> (u32, u32) {
        let index_x = if x == 0.0 { 0.0 } else { x / self.unit_width };
        let index_y = if y == 0.0 { 0.0 } else { y / self.unit_height };
        (index_x as u32, index_y as u32)
    }
}

#[cfg(test)]
mod test {
    use ggez::graphics::Rect;

    use crate::draw_system::player_draw_system::PlayerDrawSystem;
    use crate::entity::builder::EntityBuilder;
    use crate::physics_system::player_physics_system::PlayerPhysicsSystem;

    use super::*;

    #[test]
    fn ci_test_grid_instantiate() {
        let world_width = 10.0;
        let world_height = 20.0;
        let unit_width = 5.0;
        let unit_height = 10.0;
        let grid: Grid = Grid::new(world_width, world_height, unit_width, unit_height);
        let mut expected_cells = HashMap::new();
        expected_cells.insert((0, 0), vec![]);
        expected_cells.insert((1, 0), vec![]);
        expected_cells.insert((0, 1), vec![]);
        expected_cells.insert((1, 1), vec![]);
        assert_eq!(grid.cells, expected_cells);
    }

    #[test]
    fn ci_test_insert_entity_into_grid() {
        let mut grid = Grid::new(10.0, 10.0, 5.0, 5.0);
        let mut entity_builder = EntityBuilder::new();
        let entity = entity_builder.create_entity().location(3.0, 3.0).build();
        let entity_id = entity.id;
        grid.insert(&entity);
        assert_eq!(grid.cells.get(&(0, 0)).unwrap()[0], entity_id);
    }

    #[test]
    fn ci_test_query_entities_from_grid() {
        let mut grid = Grid::new(10.0, 10.0, 2.0, 2.0);
        let mut entity_builder = EntityBuilder::new();
        let query = Rect::new(0.0, 0.0, 5.0, 5.0);
        let entity_to_get = entity_builder.create_entity().location(3.5, 3.6).build();
        let entity_to_not_get = entity_builder.create_entity().location(7.8, 9.1).build();
        let id_to_get = entity_to_get.id;
        grid.insert(&entity_to_get).unwrap();
        grid.insert(&entity_to_not_get).unwrap();
        let visible_entities: Vec<&u32> = grid.query(query);
        assert_eq!(visible_entities.len(), 1);
        assert_eq!(visible_entities[0], &id_to_get);
    }

    #[test]
    fn ci_test_move_entity_between_cells_in_grid() {
        let mut grid = Grid::new(10.0, 10.0, 2.0, 2.0);
        let mut entity_builder = EntityBuilder::new();
        let mut entity = entity_builder.create_entity().location(1.5, 1.5).build();
        grid.insert(&entity);
        let old_location = entity.location;
        entity.location.x = 2.5;
        let new_location = entity.location;
        grid.update_entity_location(old_location, &entity);
        let coordinates = grid.get_coordinates(entity.location.x, entity.location.y);
        assert_eq!(grid.cells.get(&coordinates).unwrap().len(), 1);
    }
}

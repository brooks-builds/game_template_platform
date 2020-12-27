use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;

use crate::entity::{self, Entity};

use super::cell::Cell;

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    pub cells: Vec<Vec<Option<Cell>>>,
    pub cell_width: f32,
    pub cell_height: f32,
}

impl Grid {
    pub fn new(world_width: f32, world_height: f32, cell_width: f32, cell_height: f32) -> Self {
        let width = (world_width / cell_width) as usize;
        let height = (world_height / cell_height) as usize;

        let mut cells = vec![];

        for _index_y in 0..height {
            let mut row = vec![];
            for _index_x in 0..width {
                row.push(None);
            }
            cells.push(row);
        }

        Self {
            width,
            height,
            cells,
            cell_width,
            cell_height,
        }
    }

    /// Inserts an entity into the grid
    /// We are basing the position on the center of the entity
    pub fn insert(&mut self, entity: Entity) {
        let index_y = (entity.location.y / self.cell_height) as usize;
        let index_x = (entity.location.x / self.cell_width) as usize;
        if let Some(cell) = &mut self.cells[index_y][index_x] {
            cell.insert(entity);
        } else {
            let mut cell = Cell::new();
            cell.insert(entity);
            self.cells[index_y][index_x] = Some(cell);
        }
    }

    pub fn query(&self, query: Rect) -> Vec<&Entity> {
        let (start_x, start_y) = self.get_index_by_location(query.x, query.y);
        let (end_x, end_y) = self.get_index_by_location(query.x + query.w, query.y + query.h);
        let mut entities = vec![];

        for index_y in start_y..=end_y {
            for index_x in start_x..=end_x {
                if let Some(cell) = &self.cells[index_y][index_x] {
                    dbg!("found");
                    entities.push(cell.get_all());
                }
            }
        }

        entities.into_iter().flatten().collect()
    }

    pub fn get_all_entities_mut(&mut self) -> Vec<&mut Entity> {
        let mut entities = vec![];

        self.cells.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|maybe_cell| {
                if let Some(cell) = maybe_cell {
                    entities.push(cell.get_all_mut())
                };
            })
        });

        entities.into_iter().flatten().collect()
    }

    pub fn get_all_cloned(&self) -> Vec<Entity> {
        let mut entities = vec![];

        self.cells.iter().for_each(|row| {
            row.iter().for_each(|maybe_cell| {
                if let Some(cell) = maybe_cell {
                    entities.push(cell.get_all_cloned());
                }
            });
        });

        entities.into_iter().flatten().collect()
    }

    pub fn move_entity(
        &mut self,
        id: u32,
        previous_location: Vector2<f32>,
        new_location: Vector2<f32>,
    ) {
        let (old_x, old_y) = self.get_index_by_location(previous_location.x, previous_location.y);
        let (new_x, new_y) = self.get_index_by_location(new_location.x, new_location.y);

        if old_x != new_x || old_y != new_y {
            if let Some(cell) = &mut self.cells[old_y][old_x] {
                if let Some(entity) = cell.take_by_id(id) {
                    self.insert(entity);
                }
            }
        }

        if let Some(cell) = &self.cells[old_y][old_x] {
            if cell.is_empty() {
                self.cells[old_y][old_x] = None;
            }
        }
    }

    fn get_index_by_location(&self, x: f32, y: f32) -> (usize, usize) {
        (
            (x / self.cell_width) as usize,
            (y / self.cell_height) as usize,
        )
    }
}

#[cfg(test)]
mod test {
    use ggez::graphics::Rect;
    use ggez::nalgebra::Vector2;

    use crate::entity::builder::EntityBuilder;
    use crate::entity::Entity;
    use crate::physics_system::player_physics_system::PlayerPhysicsSystem;

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_create_grid() {
        let world_width = 100.0;
        let world_height = 100.0;
        let cell_width = 10.0;
        let cell_height = 10.0;
        let expected_width = 10;
        let expected_height = 10;

        let grid: Grid = Grid::new(world_width, world_height, cell_width, cell_height);
        assert_eq!(grid.width, expected_width);
        assert_eq!(grid.height, expected_height);
        assert_eq!(grid.cells.len(), 10);
        assert_eq!(grid.cells[0].len(), 10);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_insert_entity() {
        let mut grid = Grid::new(100.0, 100.0, 10.0, 10.0);
        let entity = Entity::default();
        grid.insert(entity);
        assert!(!matches!(grid.cells[0][0], None));
        if let Some(cell) = &grid.cells[0][0] {
            assert_eq!(cell.get_all()[0].location.x, 0.0);
        }
        let mut entity = Entity::new();
        entity.set_location(15.0, 15.0);
        grid.insert(entity);
        assert!(!matches!(grid.cells[1][1], None));
        if let Some(cell) = &grid.cells[1][1] {
            assert_eq!(cell.get_all()[0].location.x, 15.0);
        }
    }

    #[test]
    fn ci_test_query_for_entities() {
        let mut grid = Grid::new(100.0, 100.0, 10.0, 10.0);
        let entity_1 = Entity::new();
        let mut entity_2 = Entity::new();
        entity_2.set_location(15.0, 15.0);
        let mut entity_3 = Entity::new();
        entity_3.set_location(95.0, 90.0);
        grid.insert(entity_1);
        grid.insert(entity_2);
        grid.insert(entity_3);
        let query = Rect::new(0.0, 0.0, 50.0, 50.0);
        let entities: Vec<&Entity> = grid.query(query);
        assert_eq!(entities.len(), 2);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_get_all_cloned() {
        let mut grid = Grid::new(100.0, 100.0, 10.0, 10.0);
        let mut entity_1 = Entity::new();
        entity_1.set_physics_system(Box::new(PlayerPhysicsSystem::default()));
        grid.insert(entity_1);
        let other_entities = grid.get_all_cloned();
        assert_eq!(other_entities[0].collidable, false);
        assert_eq!(other_entities[0].location.x, 0.0);
        assert!(matches!(other_entities[0].physics_system, None));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_grid_should_move_entity_from_one_cell_to_the_next() {
        let mut grid = Grid::new(10.0, 10.0, 5.0, 5.0);
        let mut entity_builder = EntityBuilder::new();
        let player = entity_builder.create_entity().location(3.0, 4.0).build();
        assert_eq!(player.id, 0);
        grid.insert(player);
        let mut entities = &mut grid.get_all_entities_mut();
        let player = &mut entities[0];
        let old_location = player.location;
        player.location.y = 6.0;
        let cloned_player = player.clone();
        let new_location = player.location;
        grid.move_entity(0, old_location, new_location);
        assert!(grid.cells[0][0].is_none());
        let entities = grid.cells[1][0].as_ref().unwrap().get_all();
        assert_eq!(entities[0].location, Vector2::new(3.0, 6.0));
    }
}

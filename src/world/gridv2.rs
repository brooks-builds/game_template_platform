use std::collections::HashMap;

use eyre::eyre;
use eyre::Result;

use crate::Entity;

pub struct Grid {
    width: u32,
    height: u32,
    cells: HashMap<(u32, u32), Vec<u32>>,
    entities: Vec<Entity>,
}

impl Grid {
    pub fn new(world_width: f32, world_height: f32, unit_width: f32, unit_height: f32) -> Self {
        let width = (world_width / unit_width) as u32;
        let height = (world_height / unit_height) as u32;
        let entities = vec![];
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
            entities,
        }
    }

    pub fn insert(&mut self, entity: Entity) -> Result<()> {
        let coordinates = (
            self.width / entity.location.x as u32,
            self.height / entity.location.y as u32,
        );

        // if let Some(cell) = self.cells.get_mut(&coordinates) {
        //     Ok(())
        // } else {
        //     Err(BBErrors::GridCellDoesNotExist.into())
        // }

        let cell = self.cells.get_mut(&coordinates).ok_or_else(|| {
            eyre!(
                "error inserting entity into grid at coordinates: {:?}",
                coordinates
            )
        })?;

        cell.push(entity.id);
        self.entities.push(entity);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::entity::builder::EntityBuilder;

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
        grid.insert(entity);
        assert_eq!(grid.cells.get(&(0, 0)).unwrap()[0], entity_id);
        assert_eq!(grid.entities[0].id, entity_id);
    }
}

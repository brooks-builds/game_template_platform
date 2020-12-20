use ggez::graphics::Rect;

use crate::entity::Entity;

use super::cell::Cell;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(world_width: f32, world_height: f32, cell_width: f32, cell_height: f32) -> Self {
        let width = (world_width / cell_width) as usize;
        let height = (world_height / cell_height) as usize;

        let mut cells = vec![];

        for _index_y in 0..height {
            let mut row = vec![];
            for _index_x in 0..width {
                row.push(Cell::new(cell_width, cell_height));
            }
            cells.push(row);
        }

        Self {
            width,
            height,
            cells,
        }
    }

    /// Inserts an entity into the grid
    /// We are basing the position on the center of the entity
    pub fn insert(&mut self, entity: Entity) {
        let index_y = entity.location.y as usize / self.height;
        let index_x = entity.location.x as usize / self.width;
        self.cells[index_y][index_x].insert(entity);
    }

    pub fn query(&self, query: Rect) -> Vec<&Entity> {
        let start_index_x = query.x as usize / self.width;
        let start_index_y = query.y as usize / self.height;
        let end_index_x = (query.x + query.w) as usize / self.width;
        let end_index_y = (query.y + query.h) as usize / self.height;
        let mut entities = vec![];

        for index_y in start_index_y..=end_index_y {
            for index_x in start_index_x..=end_index_x {
                entities.push(self.cells[index_y][index_x].get_all());
            }
        }

        entities.into_iter().flatten().collect()
    }

    pub fn get_all_entities_mut(&mut self) -> Vec<&mut Entity> {
        let mut entities = vec![];

        self.cells.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|cell| entities.push(cell.get_all_mut()));
        });

        entities.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod test {
    use ggez::graphics::Rect;

    use crate::entity::Entity;

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
        assert_eq!(grid.cells[0][0].width, cell_width);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_insert_entity() {
        let mut grid = Grid::new(100.0, 100.0, 10.0, 10.0);
        let entity = Entity::default();
        grid.insert(entity);
        assert_eq!(grid.cells[0][0].get_all()[0].location.x, 0.0);
        let entity = Entity::default().set_location(15.0, 15.0);
        grid.insert(entity);
        assert_eq!(grid.cells[1][1].get_all()[0].location.x, 15.0);
    }

    #[test]
    fn ci_test_query_for_entities() {
        let mut grid = Grid::new(100.0, 100.0, 10.0, 10.0);
        let entity_1 = Entity::default();
        let entity_2 = Entity::default().set_location(15.0, 15.0);
        let entity_3 = Entity::default().set_location(95.0, 90.0);
        grid.insert(entity_1);
        grid.insert(entity_2);
        grid.insert(entity_3);
        let query = Rect::new(0.0, 0.0, 50.0, 50.0);
        let entities: Vec<&Entity> = grid.query(query);
        assert_eq!(entities.len(), 2);
    }
}

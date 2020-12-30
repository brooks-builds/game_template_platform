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
    pub fn insert(&mut self, entity: &Entity) {
        // let index_y = (entity.location.y / self.cell_height) as usize;
        // let index_x = (entity.location.x / self.cell_width) as usize;
        // if let Some(cell) = &mut self.cells[index_y][index_x] {
        //     cell.insert(entity);
        // } else {
        //     let mut cell = Cell::new();
        //     cell.insert(entity);
        //     self.cells[index_y][index_x] = Some(cell);
        // }
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
        // let (old_x, old_y) = self.get_index_by_location(previous_location.x, previous_location.y);
        // let (new_x, new_y) = self.get_index_by_location(new_location.x, new_location.y);

        // if old_x != new_x || old_y != new_y {
        //     if let Some(cell) = &mut self.cells[old_y][old_x] {
        //         if let Some(entity) = cell.take_by_id(id) {
        //             self.insert(entity);
        //         }
        //     }
        // }

        // if let Some(cell) = &self.cells[old_y][old_x] {
        //     if cell.is_empty() {
        //         self.cells[old_y][old_x] = None;
        //     }
        // }
    }

    fn get_index_by_location(&self, x: f32, y: f32) -> (usize, usize) {
        (
            (x / self.cell_width) as usize,
            (y / self.cell_height) as usize,
        )
    }
}

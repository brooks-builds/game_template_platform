use ggez::graphics::{Color, WHITE};

use super::entity_type::EntityType;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EntityData {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub entity_type: EntityType,
}

impl EntityData {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        entity_type: EntityType,
    ) -> Self {
        Self {
            width,
            height,
            x,
            y,
            color,
            entity_type,
        }
    }
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
            color: WHITE,
            entity_type: EntityType::Platform,
        }
    }
}

#[cfg(test)]
mod test {
    use ggez::graphics::WHITE;

    use crate::entity::entity_type::EntityType;

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_create_position() {
        let width = 10.0;
        let height = 15.0;
        let x = 5.0;
        let y = 20.0;
        let color = WHITE;
        let entity_type = EntityType::Platform;
        let entity_data = EntityData::new(x, y, width, height, color, entity_type);
        assert_eq!(entity_data.width, width);
        assert_eq!(entity_data.height, height);
        assert_eq!(entity_data.x, x);
        assert_eq!(entity_data.y, y);
        assert_eq!(entity_data.color, color);
        assert!(matches!(entity_data.entity_type, EntityType::Platform));
    }
}

use crate::entity::entity_data::EntityData;

#[derive(Debug, Clone)]
pub struct Level {
    pub width: f32,
    pub height: f32,
    pub entity_datas: Vec<EntityData>,
}

impl Level {
    pub fn new(width: f32, height: f32, entity_datas: Vec<EntityData>) -> Level {
        Self {
            width,
            height,
            entity_datas,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_create_level() {
        let entity = EntityData::default();
        let level = Level::new(10.0, 11.0, vec![entity]);
        assert_eq!(level.width, 10.0);
        assert_eq!(level.height, 11.0);
    }
}

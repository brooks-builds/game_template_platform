pub struct Config {
    pub target_update_fps: u32,
    pub gravity_force: f32,
    pub world_width: f32,
    pub world_height: f32,
    pub world_unit_width: f32,
    pub world_unit_height: f32,
}

impl Default for Config {
    fn default() -> Self {
        let target_update_fps = 50;
        let gravity_force = 0.1;
        let world_width = 5000.0;
        let world_height = 5000.0;
        let world_unit_width = 10.0;
        let world_unit_height = 10.0;

        Self {
            target_update_fps,
            gravity_force,
            world_width,
            world_height,
            world_unit_width,
            world_unit_height,
        }
    }
}

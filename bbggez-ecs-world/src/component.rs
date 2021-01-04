pub enum ComponentData {
    None,
    Point { x: f32, y: f32 },
    Radius(f32),
    Width(f32),
    Height(f32),
    Color { red: u8, green: u8, blue: u8 },
}

impl ComponentData {
    pub fn get_point(&self) -> Option<(f32, f32)> {
        match self {
            ComponentData::Point { x, y } => Some((*x, *y)),
            _ => None,
        }
    }

    pub fn get_radius(&self) -> Option<f32> {
        match self {
            ComponentData::Radius(radius) => Some(*radius),
            _ => None,
        }
    }
}

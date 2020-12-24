use ggez::graphics::Rect;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn as_rect(&self) -> Rect {
        Rect::new(
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.width,
            self.height,
        )
    }
}

#[cfg(test)]
mod test {
    use ggez::graphics::Rect;

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_creating_the_camera() {
        let x = 0.0;
        let y = 1.0;
        let width = 50.0;
        let height = 100.0;
        let camera = Camera::new(x, y, width, height);
        assert_eq!(camera.x, x);
        assert_eq!(camera.y, y);
        assert_eq!(camera.width, width);
        assert_eq!(camera.height, height);
    }

    #[test]
    fn ci_test_camera_as_rect() {
        let x = 0.0;
        let y = 1.0;
        let width = 50.0;
        let height = 100.0;
        let camera = Camera::new(x, y, width, height);
        let camera_rect: Rect = camera.as_rect();
        assert_eq!(camera_rect, Rect::new(-25.0, -49.0, 50.0, 100.0));
    }
}

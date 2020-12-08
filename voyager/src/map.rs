use euclid::Point2D;
use raqote::*;

pub struct MapSpace;
pub type MapPoint = Point2D<f64, MapSpace>;

pub struct Map {
    dt: DrawTarget,
    // points: Vec<Point2D<f64, MapSpace>>
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Map {
            dt: DrawTarget::new(width as i32, height as i32),
        }
    }

    #[inline]
    pub fn get_frame(&self) -> &[u32] {
        self.dt.get_data()
    }

    pub fn draw(&mut self) {
        self.dt
            .clear(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0));
    }
}

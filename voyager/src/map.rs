use euclid::Point2D;

pub struct MapSpace;

pub type MapPoint = Point2D<f64, MapSpace>;

pub type Map = Vec<MapPoint>;

use euclid::Point2D;
use itertools::Itertools;
use rand::prelude::*;

pub struct MapSpace;

pub type MapPoint = Point2D<f64, MapSpace>;

pub type Map = Vec<MapPoint>;

pub fn random_map(width: u32, height: u32, entities: usize) -> Map {
    let mut rng = thread_rng();
    std::iter::repeat((width, height))
        .map(|(x, y)| (rng.gen_range(0..x), rng.gen_range(0..y)))
        .unique()
        .take(entities)
        .map(|(x, y)| MapPoint::new(x as f64, y as f64))
        .collect::<Vec<MapPoint>>()
}

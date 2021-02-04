use euclid::Point2D;
use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::prelude::*;

pub struct MapSpace;

pub type MapUnit = OrderedFloat<f64>;

pub type MapPoint = Point2D<MapUnit, MapSpace>;

pub type FxIndexSet<T> = IndexSet<T, FxBuildHasher>;
pub type Map = IndexSet<MapPoint>;

pub fn random_map(width: u32, height: u32, entities: usize) -> Map {
    let mut rng = thread_rng();
    std::iter::repeat((width, height))
        .map(|(x, y)| (rng.gen_range(0..x), rng.gen_range(0..y)))
        .unique()
        .take(entities)
        .map(|(x, y)| (x as f64, y as f64))
        .map(|(x, y)| (MapUnit::from(x), MapUnit::from(y)))
        .map(|(x, y)| MapPoint::new(x, y))
        .collect::<IndexSet<MapPoint>>()
}

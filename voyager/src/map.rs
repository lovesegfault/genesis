use euclid::Point2D;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::prelude::*;

pub struct MapSpace;
pub type MapUnit = OrderedFloat<f64>;
pub type MapPoint = Point2D<MapUnit, MapSpace>;
pub type MapInner = Vec<MapPoint>;

#[derive(Clone)]
pub struct Map(pub MapInner);

impl std::ops::Deref for Map {
    type Target = MapInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<MapInner> for Map {
    fn from(inner: MapInner) -> Self {
        Self(inner)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.iter();
        let start = iter
            .next()
            .map(|p| format!("({}, {})", p.x, p.y))
            .unwrap_or_else(|| "".to_string());
        let repr = iter.fold(start, |acc, pt| format!("{} -> ({}, {})", acc, pt.x, pt.y));
        write!(f, "{}", repr)
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn random_map(width: u32, height: u32, entities: usize) -> Map {
    let mut rng = thread_rng();
    std::iter::repeat((width, height))
        .map(|(x, y)| (rng.gen_range(0..x), rng.gen_range(0..y)))
        .unique()
        .take(entities)
        .map(|(x, y)| (x as f64, y as f64))
        .map(|(x, y)| (MapUnit::from(x), MapUnit::from(y)))
        .map(|(x, y)| MapPoint::new(x, y))
        .collect::<MapInner>()
        .into()
}

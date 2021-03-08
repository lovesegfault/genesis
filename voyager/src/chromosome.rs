use crate::map::{Map, MapPoint};
use itertools::Itertools;
use rand::prelude::*;
// use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Chromosome {
    pub solution: Map,
    pub score: f64,
}

impl Chromosome {
    #[inline]
    pub fn new(solution: Map) -> Self {
        let score = Self::score(&solution);
        Chromosome { solution, score }
    }

    pub fn random(source: &Map) -> Self {
        let mut solution = source.iter().copied().collect::<Vec<_>>();
        solution.shuffle(&mut thread_rng());
        Chromosome::new(solution)
    }

    #[inline(always)]
    fn score(path: &Map) -> f64 {
        let cost: f64 = path
            .iter()
            .map(|point| (point.x.into_inner(), point.y.into_inner()))
            .map(|(x, y)| -> euclid::Point2D<f64, ()> { euclid::Point2D::new(x, y) })
            .tuple_windows::<(_, _)>()
            .map(|subpath| subpath.0.distance_to(subpath.1))
            .sum();
        1.0 / cost
    }

    // FIXME: It'd be nice if we could just take a reference to the parents instead
    pub fn crossover(self, other: Self) -> (Self, Self) {
        // First we clone the father and mother strings.
        // We only need to clone so that we can rotate later. I'd like to get rid of this.
        let mut father = self.solution;
        let mut mother = other.solution;

        debug_assert_eq!(father.len(), mother.len());
        let len = father.len();

        // Now we pick two random cutting points which will be identical for the father and mother
        let mut rng = thread_rng();
        let mut min: usize = 0;
        let mut max: usize = 0;
        while min == max {
            min = rng.gen_range(0..len);
            max = rng.gen_range(0..len);
        }
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        // Prepare to construct the offsprint from the crossover
        let mut son = vec![MapPoint::default(); len];
        let mut daughter = vec![MapPoint::default(); len];

        // Align the parents so we can drain the copy-over portion
        father.rotate_left(min);
        mother.rotate_left(min);
        let drain_limit = max - min;

        // Copy the middle portions as-is
        son[min..max].copy_from_slice(&mother.drain(0..drain_limit).as_slice());
        daughter[min..max].copy_from_slice(&father.drain(0..drain_limit).as_slice());

        // Filter nodes already present in offspring from parents (swapped parents wrt the middle
        // copy)
        dbg!(&father);
        dbg!(&mother);
        dbg!(&son);
        dbg!(&daughter);
        dbg!("filtering mids");
        father.retain(|point| !son.contains(point));
        mother.retain(|point| !daughter.contains(point));
        dbg!("ok");
        dbg!(&father);
        dbg!(&mother);

        // Finally, copy over remaining nodes
        // The lower portion
        dbg!("copying upper");
        son[0..min].copy_from_slice(&father.drain(0..min).as_slice());
        daughter[0..min].copy_from_slice(&mother.drain(0..min).as_slice());
        dbg!("ok");
        // The upper portion
        dbg!("copying lower");
        son[max..].copy_from_slice(&father.drain(..).as_slice());
        son[max..].copy_from_slice(&mother.drain(..).as_slice());
        dbg!("ok");

        let mut son = Chromosome::new(son);
        let mut daughter = Chromosome::new(daughter);

        // Lastly, we randomly mutate the children before returning
        son.mutate();
        daughter.mutate();

        (son, daughter)
    }

    #[inline]
    fn mutate(&mut self) {
        use rand::distributions::Uniform;

        let mut rng = thread_rng();

        dbg!(self.solution.len());
        let index_distribution = Uniform::from(0..self.solution.len());
        let rand_maybe = rng.gen_range(0..100);

        if rand_maybe <= 80 {
            for _ in 0..3 {
                let a = index_distribution.sample(&mut rng);
                let b = index_distribution.sample(&mut rng);
                self.solution.swap(a, b)
            }
        }

        self.score = Self::score(&self.solution);
    }
}

impl std::fmt::Display for Chromosome {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.solution.iter();
        let start = iter
            .next()
            .map(|p| format!("({}, {})", p.x, p.y))
            .unwrap_or_else(|| "".to_string());
        let repr = iter.fold(start, |acc, pt| format!("{} -> ({}, {})", acc, pt.x, pt.y));
        write!(fmt, "{}", repr)
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Chromosome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or_else(|| std::cmp::Ordering::Less)
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Chromosome {}

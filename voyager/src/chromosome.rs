use crate::map::{Map, MapPoint, MapSpace};
use rand::prelude::*;
use rayon::prelude::*;

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
        Chromosome::new(solution.into())
    }

    #[inline(always)]
    fn score(path: &Map) -> f64 {
        fn repack_point<P: Into<f64>>(
            p: euclid::Point2D<P, MapSpace>,
        ) -> euclid::Point2D<f64, MapSpace> {
            let x = p.x.into();
            let y = p.y.into();
            euclid::Point2D::new(x, y)
        }
        let mut cost: f64 = path
            .0
            .par_windows(2)
            .map(|window| (repack_point(window[0]), repack_point(window[1])))
            .map(|subpath| subpath.0.distance_to(subpath.1))
            .sum();
        let start = repack_point(path.0[0]);
        let end = repack_point(*path.0.last().unwrap());
        cost += end.distance_to(start);
        1.0 / cost
    }

    pub fn crossover(self, other: Self) -> (Self, Self) {
        // First we clone the father and mother strings.
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
        let mut son = Map::from(vec![MapPoint::default(); len]);
        let mut daughter = Map::from(vec![MapPoint::default(); len]);

        // Copy the middle portions as-is
        son[min..max].copy_from_slice(&mother[min..max]);
        daughter[min..max].copy_from_slice(&father[min..max]);

        // Rotate parents so they start after the maximum cut-point
        father.rotate_left(max);
        mother.rotate_left(max);

        // Filter nodes already present in offspring from parents (swapped parents wrt the middle
        // copy)
        father.retain(|point| !son.contains(point));
        mother.retain(|point| !daughter.contains(point));

        // Finally, copy over remaining nodes
        // The upper portion
        let upper_cut = len - max;
        son[max..].copy_from_slice(&father.drain(0..upper_cut).as_slice());
        daughter[max..].copy_from_slice(&mother.drain(0..upper_cut).as_slice());
        // The lower portion
        son[..min].copy_from_slice(&father.drain(..).as_slice());
        daughter[..min].copy_from_slice(&mother.drain(..).as_slice());

        (son.into(), daughter.into())
    }

    #[inline]
    pub fn mutate(&mut self) {
        use rand::distributions::Uniform;
        let mut rng = thread_rng();
        let rand_maybe = rng.gen_range(0..100);

        // We only mutate with a 80% probability
        if rand_maybe <= 80 {
            let mut mutated = self.clone();
            let index_distribution = Uniform::from(0..self.solution.len());
            let swaps = rng.gen_range(0..(self.solution.len() / 2));
            for _ in 0..swaps {
                let a = index_distribution.sample(&mut rng);
                let b = index_distribution.sample(&mut rng);
                mutated.solution.swap(a, b)
            }
            mutated.score = Self::score(&mutated.solution);

            // We allow worse mutations to survive 10% of the time
            if mutated.score < self.score || rand_maybe < 10 {
                std::mem::swap(self, &mut mutated);
            }
        }
    }
}

impl From<Map> for Chromosome {
    fn from(map: Map) -> Self {
        Self::new(map)
    }
}

impl std::fmt::Display for Chromosome {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.solution)
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
            .unwrap_or(std::cmp::Ordering::Less)
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Chromosome {}

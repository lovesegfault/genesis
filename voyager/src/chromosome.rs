use crate::map::{Map, MapPoint};
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

    pub fn random(points: &[MapPoint]) -> Self {
        let mut solution = points.to_vec();
        solution.shuffle(&mut thread_rng());
        Chromosome::new(solution.into())
    }

    #[inline(always)]
    fn score(path: &[MapPoint]) -> f64 {
        let cost: f64 = path
            .windows(2)
            .map(|subpath| subpath[0].distance_to(subpath[1]))
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
        // e.g. 'Twas brill|ig, and the slithy |toves
        // | denotes the cut
        // min = 11
        // max = 31
        let mut rng = thread_rng();
        let mut min: usize = rng.gen_range(0..len);
        let mut max: usize = rng.gen_range(0..len);
        while min == max {
            max = rng.gen_range(0..len);
        }
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        let mut son = vec![MapPoint::default(); len];
        let mut daughter = vec![MapPoint::default(); len];

        // The chunk of text between the cut points is copied verbatim to the children
        son[min..max].copy_from_slice(&mother[min..max]);
        daughter[min..max].copy_from_slice(&father[min..max]);

        // We fill the remaining gaps in the children as if the parent slice was rotated. To avoid
        // mutating the parent slice, we use this bespoke copy_slice_rotated
        let max_gap = len - max;
        let min_gap = max_gap + min;

        // Fill the remaining gaps in the children with elements from the parents,
        // starting from the portion following the transplanted section
        // copy_slice_rotated(&father, rot_left, 0..max_gap, &mut son[max..len]);
        // copy_slice_rotated(&father, rot_left, max_gap..min_gap, &mut son[0..min]);
        // copy_slice_rotated(&mother, rot_left, 0..max_gap, &mut daughter[max..len]);
        // copy_slice_rotated(&mother, rot_left, max_gap..min_gap, &mut daughter[0..min]);

        let mut son = Chromosome::new(son.into());
        let mut daughter = Chromosome::new(daughter.into());

        // Lastly, we randomly mutate the children before returning
        son.mutate();
        daughter.mutate();

        (son, daughter)
    }

    #[inline]
    fn mutate(&mut self) {
        use rand::distributions::Uniform;

        let mut rng = thread_rng();

        let mut mutated = self.solution.clone();
        let index_distribution = Uniform::from(0..self.solution.len());
        let rand_maybe = rng.gen_range(0..100);

        if rand_maybe <= 80 {
            for _ in 0..3 {
                let a = index_distribution.sample(&mut rng);
                let b = index_distribution.sample(&mut rng);
                mutated.swap(a, b);
            }
        }

        let mutate_score = Self::score(&mutated);

        if mutate_score > self.score || rand_maybe < 20 {
            self.solution = mutated;
            self.score = mutate_score;
        }
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

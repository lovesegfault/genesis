use rand::prelude::*;
use std::ops::Range;

fn copy_slice_rotated<T: Copy>(
    src: &[T],
    left_rotation: usize,
    Range { start, end }: Range<usize>,
    dst: &mut [T],
) {
    let len = src.len();
    let gap = end - start;
    let rot_start = (start + left_rotation) % len;
    let rot_end = rot_start + gap;

    if rot_end <= len {
        dst.copy_from_slice(&src[rot_start..rot_end]);
    } else {
        let first = &src[rot_start..];
        let second = &src[..(rot_end % len)];
        dst[..first.len()].copy_from_slice(first);
        dst[first.len()..].copy_from_slice(second);
    }
}

#[derive(Clone, Debug)]
pub struct Chromosome<'g> {
    pub solution: Vec<u8>,
    pub cost: u32,
    goal: &'g [u8],
}

impl<'g> Chromosome<'g> {
    #[inline]
    pub fn new(solution: Vec<u8>, goal: &'g [u8]) -> Self {
        let cost = Self::distance(&solution, goal);
        Chromosome {
            solution,
            cost,
            goal,
        }
    }

    pub fn random(goal: &'g [u8]) -> Self {
        let solution: Vec<u8> = std::iter::repeat_with(random).take(goal.len()).collect();
        let cost = Self::distance(&solution, goal);
        Chromosome {
            solution,
            cost,
            goal,
        }
    }

    #[inline(always)]
    fn distance(a: &[u8], b: &[u8]) -> u32 {
        triple_accel::hamming(a, b)
    }

    pub fn crossover(&self, other: &Self) -> (Self, Self) {
        // First we clone the father and mother strings.
        // We only need to clone so that we can rotate later. I'd like to get rid of this.
        let father = &self.solution;
        let mother = &other.solution;

        debug_assert_eq!(father.len(), mother.len());
        let len = father.len();

        // Now we pick two random cutting points which will be identical for the father and mother
        // e.g. 'Twas brill|ig, and the slithy |toves
        // | denotes the cut
        // min = 11
        // max = 31
        let mut rng = thread_rng();
        let mut min: usize = rng.gen_range(0, len);
        let mut max: usize = rng.gen_range(0, len);
        while min == max {
            max = rng.gen_range(0, len);
        }
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        let mut son = vec![0; len];
        let mut daughter = vec![0; len];

        // The chunk of text between the cut points is copied verbatim to the children
        son[min..max].copy_from_slice(&mother[min..max]);
        daughter[min..max].copy_from_slice(&father[min..max]);

        // We fill the remaining gaps in the children as if the parent slice was rotated. To avoid
        // mutating the parent slice, we use this bespoke copy_slice_rotated
        let rot_left = max;
        let max_gap = len - max;
        let min_gap = max_gap + min;

        // Fill the remaining gaps in the children with elements from the parents,
        // starting from the portion following the transplanted section
        copy_slice_rotated(&father, rot_left, 0..max_gap, &mut son[max..len]);
        copy_slice_rotated(&father, rot_left, max_gap..min_gap, &mut son[0..min]);
        copy_slice_rotated(&mother, rot_left, 0..max_gap, &mut daughter[max..len]);
        copy_slice_rotated(&mother, rot_left, max_gap..min_gap, &mut daughter[0..min]);

        let mut son = Chromosome::new(son, self.goal);
        let mut daughter = Chromosome::new(daughter, self.goal);

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
        let rand_maybe = rng.gen_range(0, 100);

        if rand_maybe <= 80 {
            for _ in 0..3 {
                mutated[index_distribution.sample(&mut rng)] = random();
            }
        }

        let mutate_cost = Self::distance(&mutated, self.goal);

        if mutate_cost < self.cost || rand_maybe < 20 {
            self.solution = mutated;
            self.cost = mutate_cost;
        }
    }
}

impl std::fmt::Display for Chromosome<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", String::from_utf8_lossy(&self.solution))
    }
}

impl PartialOrd for Chromosome<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for Chromosome<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Chromosome<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Chromosome<'_> {}

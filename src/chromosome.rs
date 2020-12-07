use rand::prelude::*;

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
        let mut father = self.solution.clone();
        let mut mother = other.solution.clone();

        debug_assert_eq!(father.len(), mother.len());
        let len = father.len();

        let mut rng = thread_rng();
        let mut min: usize = rng.gen_range(0, len);
        let mut max: usize = rng.gen_range(0, len);
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        let mut son = vec![0; len];
        let mut daughter = vec![0; len];

        son[min..max].copy_from_slice(&mother[min..max]);
        daughter[min..max].copy_from_slice(&father[min..max]);

        mother.rotate_left(max);
        father.rotate_left(max);

        let max_gap = len - max;
        let min_gap = max_gap - min;

        son[max..len].copy_from_slice(&father[0..max_gap]);
        son[0..min].copy_from_slice(&father[max_gap..min_gap]);

        daughter[max..len].copy_from_slice(&mother[0..max_gap]);
        daughter[0..min].copy_from_slice(&mother[max_gap..min_gap]);

        let mut son = Chromosome::new(son, self.goal);
        let mut daughter = Chromosome::new(daughter, self.goal);

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

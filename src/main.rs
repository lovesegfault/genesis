use rand::prelude::*;
use std::ops::Mul;

#[derive(Clone, Debug)]
struct Chromosome {
    solution: Vec<u8>,
}

unsafe impl Send for Chromosome {}

impl Chromosome {
    fn random(len: usize) -> Self {
        let seed = std::iter::repeat(()).map(|_| random()).take(len).collect();

        Chromosome { solution: seed }
    }

    fn distance(&self, goal: &[u8]) -> u32 {
        triple_accel::levenshtein_exp(&self.solution, goal)
    }

    fn mutate(&mut self) {
        use rand::distributions::Uniform;

        let mut rng = thread_rng();

        let index_distribution = Uniform::from(0..self.solution.len());
        let rand_maybe = Uniform::from(0..100).sample(&mut rng);

        if rand_maybe <= 20 {
            self.solution[index_distribution.sample(&mut rng)] = rng.gen();
            self.solution[index_distribution.sample(&mut rng)] = rng.gen();
        }

        if rand_maybe <= 2 {
            self.solution[index_distribution.sample(&mut rng)] = rng.gen();
            self.solution[index_distribution.sample(&mut rng)] = rng.gen();
        }
    }
}

impl std::fmt::Display for Chromosome {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "\"{}\"", String::from_utf8_lossy(&self.solution))
    }
}

impl Mul for Chromosome {
    type Output = (Self, Self);

    fn mul(self, rhs: Self) -> Self::Output {
        use rand::distributions::Uniform;

        let mut father = self.solution;
        let mut mother = rhs.solution;

        assert_eq!(father.len(), mother.len());
        let len = father.len();

        let distribution = Uniform::from(0..len);
        let mut rng = thread_rng();
        let cut_a: usize = distribution.sample(&mut rng);
        let cut_b: usize = distribution.sample(&mut rng);
        let min = cut_a.min(cut_b);
        let max = cut_a.max(cut_b);

        let mut son = vec![0; len];
        let mut daughter = vec![0; len];

        son[min..max].copy_from_slice(&mother[min..max]);
        daughter[min..max].copy_from_slice(&father[min..max]);

        mother.rotate_left(max);
        father.rotate_left(max);

        son[max..len].copy_from_slice(&mut father.drain(0..(len - max)).as_slice());
        son[0..min].copy_from_slice(&mut father.drain(0..min).as_slice());

        daughter[max..len].copy_from_slice(&mut mother.drain(0..(len - max)).as_slice());
        daughter[0..min].copy_from_slice(&mut mother.drain(0..min).as_slice());

        (
            Chromosome { solution: son },
            Chromosome { solution: daughter },
        )
    }
}

fn main() {
    use rayon::prelude::*;

    let generations = 100_000;
    let goal: Vec<u8> = b"Hello, World!".to_vec();
    let generation_size = 50;
    let parents_survive = generation_size / 10;

    let mut parents: Vec<Chromosome> = std::iter::repeat(())
        .map(|_| Chromosome::random(goal.len()))
        .take(generation_size)
        .collect();
    let mut children: Vec<Chromosome> = Vec::with_capacity(generation_size);

    for _ in 0..generations {
        parents.par_sort_unstable_by_key(|c| c.distance(&goal));

        // copy the most successful ones
        // N.B. this copies them over in reverse order.
        parents
            .iter()
            .take(parents_survive)
            .for_each(|p| children.push((*p).clone()));

        // pair parents up
        // FIXME: These should be picked with their scores in mind, but I got stuck trying to use
        // rand::distributions::WeightedIndex, so they are just picked at random.
        let mut rng = thread_rng();

        // FIXME: this is disgusting
        let mut prospects = parents
            .par_chunks_exact(2)
            .map(|p| p[0].clone() * p[1].clone())
            .flat_map(|(a, b)| rayon::iter::once(a).chain(rayon::iter::once(b)))
            .collect::<Vec<Chromosome>>();
        prospects.shuffle(&mut rng);

        prospects.par_iter_mut().for_each(|p| p.mutate());

        let remainder = generation_size - parents_survive;
        children.extend_from_slice(prospects.drain(0..remainder).as_slice());
        std::mem::swap(&mut parents, &mut children);
        children.clear();
    }

    println!("{}", parents[0]);
}

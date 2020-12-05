use rand::prelude::*;
use std::ops::Mul;

#[derive(Clone, Debug)]
struct Chromosome {
    solution: Vec<u8>,
}

impl Chromosome {
    fn random(len: usize) -> Self {
        Chromosome {
            solution: std::iter::repeat_with(|| random()).take(len).collect(),
        }
    }

    fn distance(&self, goal: &[u8]) -> u32 {
        triple_accel::hamming(&self.solution, goal)
    }

    fn mutate(&mut self) {
        use rand::distributions::Uniform;

        let mut rng = thread_rng();

        let index_distribution = Uniform::from(0..self.solution.len());
        let rand_maybe = rng.gen_range(0, 100);

        if rand_maybe <= 20 {
            self.solution[index_distribution.sample(&mut rng)] = random();
            self.solution[index_distribution.sample(&mut rng)] = random();
        }

        if rand_maybe <= 7 {
            self.solution.shuffle(&mut rng);
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
        let mut father = self.solution;
        let mut mother = rhs.solution;

        assert_eq!(father.len(), mother.len());
        let len = father.len();

        let mut rng = thread_rng();
        let cut_a: usize = rng.gen_range(0, len);
        let cut_b: usize = rng.gen_range(0, len);
        let min = cut_a.min(cut_b);
        let max = cut_a.max(cut_b);

        let mut son = vec![0; len];
        let mut daughter = vec![0; len];

        son[min..max].copy_from_slice(&mother[min..max]);
        daughter[min..max].copy_from_slice(&father[min..max]);

        mother.rotate_left(max);
        father.rotate_left(max);

        son[max..len].copy_from_slice(father.drain(0..(len - max)).as_slice());
        son[0..min].copy_from_slice(father.drain(0..min).as_slice());

        daughter[max..len].copy_from_slice(mother.drain(0..(len - max)).as_slice());
        daughter[0..min].copy_from_slice(mother.drain(0..min).as_slice());

        (
            Chromosome { solution: son },
            Chromosome { solution: daughter },
        )
    }
}

fn main() {
    use indicatif::{ProgressBar, ProgressStyle};
    use rayon::prelude::*;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["◜", "◠", "◝", "◞", "◡", "◟"])
            .template("{spinner:.blue} | {elapsed_precise} | {per_sec} | {wide_msg}"),
    );

    let goal: Vec<u8> = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse feugiat, libero eget faucibus consequat, orci erat ornare neque, faucibus iaculis sapien nunc vitae arcu.".to_vec();
    let generation_size = 50;
    let parents_survive = generation_size / 10;

    let mut parents: Vec<(Chromosome, u32)> =
        std::iter::repeat_with(|| Chromosome::random(goal.len()))
            .take(generation_size)
            .map(|c| {
                let score = c.distance(&goal);
                (c, score)
            })
            .collect();
    let mut children: Vec<(Chromosome, u32)> = Vec::with_capacity(generation_size);

    for generation in 0.. {
        if let Some(result) = parents.iter().find(|(_, score)| *score == 0) {
            pb.finish_with_message(&format!(
                "{} | {}",
                generation,
                result.0.to_string().escape_default()
            ));
            return;
        }
        parents.par_sort_unstable_by(|a, b| a.1.cmp(&b.1));

        // copy the most successful ones
        children.extend_from_slice(&parents[0..parents_survive]);

        // pair parents up
        // FIXME: These should be picked with their scores in mind, but I got stuck trying to use
        // rand::distributions::WeightedIndex, so they are just picked at random.
        // FIXME: this is disgusting
        let remainder = generation_size - parents_survive;
        parents.shuffle(&mut thread_rng());
        let mut prospects: Vec<(Chromosome, u32)> = parents
            .par_chunks_exact(2)
            .map(|p| p[0].0.clone() * p[1].0.clone())
            .flat_map(|(a, b)| rayon::iter::once(a).chain(rayon::iter::once(b)))
            .map(|mut c| {
                c.mutate();
                let score = c.distance(&goal);
                (c, score)
            })
            .collect();
        prospects.shuffle(&mut thread_rng());

        children.extend_from_slice(prospects.drain(0..remainder).as_slice());
        std::mem::swap(&mut parents, &mut children);
        children.clear();

        pb.set_message(&format!(
            "{} | {}",
            generation,
            parents[0].0.to_string().escape_default()
        ));
        pb.inc(1);
    }
}

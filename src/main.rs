#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use rand::prelude::*;

static GOAL: &[u8] = b"
Twas brillig, and the slithy toves
      Did gyre and gimble in the wabe:
All mimsy were the borogoves,
      And the mome raths outgrabe.

\"Beware the Jabberwock, my son!
      The jaws that bite, the claws that catch!
Beware the Jubjub bird, and shun
      The frumious Bandersnatch!\"

He took his vorpal sword in hand;
      Long time the manxome foe he sought-
So rested he by the Tumtum tree
      And stood awhile in thought.

And, as in uffish thought he stood,
      The Jabberwock, with eyes of flame,
Came whiffling through the tulgey wood,
      And burbled as it came!

One, two! One, two! And through and through
      The vorpal blade went snicker-snack!
He left it dead, and with its head
      He went galumphing back.

\"And hast thou slain the Jabberwock?
      Come to my arms, my beamish boy!
O frabjous day! Callooh! Callay!\"
      He chortled in his joy.

'Twas brillig, and the slithy toves
      Did gyre and gimble in the wabe:
All mimsy were the borogoves,
      And the mome raths outgrabe.
";

#[derive(Clone, Debug, Eq)]
struct Chromosome {
    solution: Vec<u8>,
    cost: u32,
}

impl Chromosome {
    fn random(len: usize) -> Self {
        let solution: Vec<u8> = std::iter::repeat_with(random).take(len).collect();
        let cost = Self::hamming(&solution);
        Chromosome { solution, cost }
    }

    fn hamming(a: &[u8]) -> u32 {
        triple_accel::hamming(a, &GOAL)
    }

    fn crossover(&self, other: &Self) -> (Self, Self) {
        let mut father = self.solution.clone();
        let mut mother = other.solution.clone();

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

        let son_cost = Chromosome::hamming(&son);
        let daughter_cost = Chromosome::hamming(&daughter);

        let mut son = Chromosome {
            solution: son,
            cost: son_cost,
        };
        let mut daughter = Chromosome {
            solution: daughter,
            cost: daughter_cost,
        };

        son.mutate();
        daughter.mutate();

        (son, daughter)
    }

    fn mutate(&mut self) {
        use rand::distributions::Uniform;

        let mut rng = thread_rng();

        let mut mutated = self.solution.clone();
        let index_distribution = Uniform::from(0..self.solution.len());
        let rand_maybe = rng.gen_range(0, 100);

        if rand_maybe <= 80 {
            mutated[index_distribution.sample(&mut rng)] = random();
            mutated[index_distribution.sample(&mut rng)] = random();
        }

        let mutate_cost = Self::hamming(&mutated);

        if mutate_cost < self.cost || rand_maybe < 20 {
            self.solution = mutated;
            self.cost = mutate_cost;
        }
    }
}

impl std::fmt::Display for Chromosome {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "\"{}\"", String::from_utf8_lossy(&self.solution))
    }
}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for Chromosome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn main() {
    use indicatif::{ProgressBar, ProgressStyle};
    use rayon::prelude::*;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner().template("{elapsed_precise} | {per_sec} | {wide_msg}"),
    );

    let generation_size = 4096;
    assert_eq!(generation_size % 2, 0);
    let parents_survive = generation_size / 10;

    let mut parents: Vec<Chromosome> = std::iter::repeat_with(|| Chromosome::random(GOAL.len()))
        .take(generation_size)
        .collect();
    let mut children: Vec<Chromosome> = Vec::with_capacity(generation_size);

    for generation in 0.. {
        if let Some(result) = parents.iter().find(|c| c.cost == 0) {
            pb.finish();
            println!("{}", result.to_string());
            return;
        }

        // copy the most successful ones
        parents.par_sort_unstable();
        children.extend_from_slice(&parents[0..parents_survive]);

        // pair parents up
        use rand::distributions::WeightedIndex;

        let remainder = generation_size - parents_survive;
        let cost: Vec<f64> = parents.iter().map(|c| 1.0 / (c.cost as f64)).collect();
        let dist = WeightedIndex::new(&cost).unwrap();

        children.par_extend(
            (0..(remainder / 2))
                .into_par_iter()
                .map(|_| {
                    let mut local_rng = thread_rng();
                    let a = dist.sample(&mut local_rng);
                    let mut b = dist.sample(&mut local_rng);
                    while a == b {
                        b = dist.sample(&mut local_rng);
                    }

                    parents[a].crossover(&parents[b])
                })
                .flat_map(|(a, b)| rayon::iter::once(a).chain(rayon::iter::once(b))),
        );

        std::mem::swap(&mut parents, &mut children);
        children.clear();

        pb.set_message(&format!(
            "{} | {}",
            generation,
            parents[0].to_string().escape_default()
        ));
        pb.inc(1);
    }
}

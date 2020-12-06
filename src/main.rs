#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod chromosome;

use chromosome::Chromosome;
use rand::{distributions::WeightedIndex, prelude::*};

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

    let mut parents: Vec<Chromosome> = std::iter::repeat_with(|| Chromosome::random(&GOAL))
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

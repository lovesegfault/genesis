#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod chromosome;

use chromosome::Chromosome;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{distributions::WeightedIndex, prelude::*};
use rayon::prelude::*;
use std::io::{stdin, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dada", about = "A genetic approach to guessing strings.")]
struct Opt {
    #[structopt(short, long, default_value = "4096")]
    generation_size: usize,
    #[structopt(short, long, default_value = "0.1")]
    parent_survival_rate: f64,
    #[structopt(short, long, default_value = "128")]
    substring_length: usize,
    #[structopt(default_value = "-")]
    text_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    assert_eq!(opt.generation_size % 2, 0);
    let generation_size = opt.generation_size;
    assert!((0.0..1.0).contains(&opt.parent_survival_rate));
    let parents_survive = opt.generation_size / (opt.parent_survival_rate * 100.0) as usize;

    let goal = if opt.text_file == PathBuf::from("-") {
        eprintln!("Reading from stdin");
        let mut input = String::new();
        stdin().read_to_string(&mut input).unwrap();
        input.into_bytes()
    } else {
        let mut file = std::fs::File::open(opt.text_file).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        text.into_bytes()
    };

    let pb = ProgressBar::new_spinner().with_style(
        ProgressStyle::default_spinner().template("{elapsed_precise} | {per_sec} | {wide_msg}"),
    );

    let solution: Vec<u8> = goal.par_chunks(opt.substring_length).flat_map(|substring| {
        let mut parents: Vec<Chromosome> =
            std::iter::repeat_with(|| Chromosome::random(substring))
                .take(generation_size)
                .collect();
        let mut children: Vec<Chromosome> = Vec::with_capacity(generation_size);

        loop {
            pb.set_message(&parents[0].to_string().escape_default().to_string());
            // copy the most successful ones
            parents.par_sort_unstable();
            children.extend_from_slice(&parents[0..parents_survive]);

            if parents[0].cost == 0 {
                break;
            }

            // pair parents up
            let remainder = generation_size - parents_survive;
            let cost: Vec<f64> = parents.iter().map(|c| 1.0 / (c.cost as f64)).collect();
            let dist = WeightedIndex::new(&cost).unwrap_or_else(|_| unsafe {
                // SAFETY: This can only panic when a weight is negative. We know the cost is always positive,
                // and 1 / (n > 0) is never < 0.
                std::hint::unreachable_unchecked();
            });

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

            pb.inc(1);
        }

        parents[0].solution.clone()
        })
        .collect();

    pb.finish();
    println!("{}", String::from_utf8_lossy(&solution));
}

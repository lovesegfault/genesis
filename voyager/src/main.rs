mod chromosome;
mod map;

use chromosome::Chromosome;
use rand::{distributions::WeightedIndex, prelude::*};
use rayon::prelude::*;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

const GENERATION_SIZE: usize = 256;
const PARENT_SURVIVAL_RATE: f64 = 0.1;
const PARENTS_SUVIVE: usize = GENERATION_SIZE / (PARENT_SURVIVAL_RATE * 100.0) as usize;

const GRID_CELL_SIZE: i32 = 5;
const GRID_WIDTH: i32 = 300;
const GRID_HEIGHT: i32 = 400;
const WINDOW_WIDTH: i32 = GRID_WIDTH * GRID_CELL_SIZE + GRID_CELL_SIZE;
const WINDOW_HEIGHT: i32 = GRID_HEIGHT * GRID_CELL_SIZE + GRID_CELL_SIZE;
const COLOR_BACKGROUND: Color = Color::RGBA(10, 14, 20, 255);
const COLOR_ENTITY: Color = Color::RGBA(230, 180, 80, 255);
const COLOR_PATH: Color = Color::RGBA(89, 194, 255, 255);

// N.B. Trait hygiene means this (sadly) can't be a From impl
fn point_to_rect(point: &map::MapPoint) -> Rect {
    Rect::new(
        point.x as i32,
        point.y as i32,
        GRID_CELL_SIZE as u32,
        GRID_CELL_SIZE as u32,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut travel_map = map::random_map(
        (WINDOW_WIDTH - GRID_CELL_SIZE) as u32,
        (WINDOW_HEIGHT - GRID_CELL_SIZE) as u32,
        20,
    );
    let mut parents: Vec<Chromosome> = std::iter::repeat_with(|| Chromosome::random(&travel_map))
        .take(GENERATION_SIZE)
        .collect();
    let mut children: Vec<Chromosome> = Vec::with_capacity(GENERATION_SIZE);

    sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "1");

    let ctx = sdl2::init()?;

    let mut event_pump = ctx.event_pump()?;

    let video_subsys = ctx.video()?;

    let gl_attr = video_subsys.gl_attr();
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(8);

    let window = video_subsys
        .window("voyager", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .opengl()
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(COLOR_BACKGROUND);
        canvas.clear();

        canvas.set_draw_color(COLOR_ENTITY);
        parents[0]
            .solution
            .iter()
            .map(point_to_rect)
            .map(|rect| canvas.fill_rect(rect))
            .collect::<Result<(), String>>()?;

        canvas.set_draw_color(COLOR_PATH);

        parents[0]
            .solution
            .windows(2)
            .map(|p| (point_to_rect(&p[0]), point_to_rect(&p[1])))
            .map(|p| (p.0.center(), p.1.center()))
            .map(|(a, b)| canvas.draw_line(a, b))
            .collect::<Result<(), String>>()?;

        canvas.present();

        parents.sort_unstable();
        children.extend_from_slice(&parents[0..PARENTS_SUVIVE]);

        let remainder = GENERATION_SIZE - PARENTS_SUVIVE;
        let score: Vec<f64> = parents.iter().map(|c| 1.0 / (c.score as f64)).collect();
        let dist = WeightedIndex::new(&score)?;

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

                    parents[a].clone().crossover(parents[b].clone())
                })
                .flat_map(|(a, b)| rayon::iter::once(a).chain(rayon::iter::once(b))),
        );

        std::mem::swap(&mut parents, &mut children);
        children.clear();
    }
    Ok(())
}

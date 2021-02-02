mod chromosome;
mod map;

use itertools::Itertools;
use rand::prelude::*;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

const GRID_CELL_SIZE: i32 = 5;
const GRID_WIDTH: i32 = 300;
const GRID_HEIGHT: i32 = 400;
const WINDOW_WIDTH: i32 = GRID_WIDTH * GRID_CELL_SIZE + GRID_CELL_SIZE;
const WINDOW_HEIGHT: i32 = GRID_HEIGHT * GRID_CELL_SIZE + GRID_CELL_SIZE;
const COLOR_BACKGROUND: Color = Color::RGBA(10, 14, 20, 255);
const COLOR_ENTITY: Color = Color::RGBA(230, 180, 80, 255);
const COLOR_PATH: Color = Color::RGBA(89, 194, 255, 255);

fn gen_entities() -> Vec<Rect> {
    let mut rng = thread_rng();
    std::iter::repeat((
        WINDOW_WIDTH - GRID_CELL_SIZE,
        WINDOW_HEIGHT - GRID_CELL_SIZE,
    ))
    .map(|(x, y)| (rng.gen_range(0..x), rng.gen_range(0..y)))
    .unique()
    .take(50)
    .map(|(x, y)| Rect::new(x, y, GRID_CELL_SIZE as u32, GRID_CELL_SIZE as u32))
    .collect::<Vec<Rect>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut entities = gen_entities();

    let ctx = sdl2::init()?;

    sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "1");

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
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    entities = gen_entities();
                }
                _ => {}
            }
        }
        canvas.set_draw_color(COLOR_BACKGROUND);
        canvas.clear();

        canvas.set_draw_color(COLOR_ENTITY);
        canvas.fill_rects(&entities)?;

        canvas.set_draw_color(COLOR_PATH);
        entities
            .windows(2)
            .map(|p| (p[0].center(), p[1].center()))
            .map(|(a, b)| canvas.draw_line(a, b))
            .collect::<Result<(), String>>()?;
        canvas.present();
    }

    Ok(())
}

mod chromosome;
mod map;

use itertools::Itertools;
use rand::prelude::*;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

const GRID_CELL_SIZE: i32 = 5;
const GRID_WIDTH: i32 = 300;
const GRID_HEIGHT: i32 = 400;
const WINDOW_WIDTH: i32 = GRID_WIDTH * GRID_CELL_SIZE + 1;
const WINDOW_HEIGHT: i32 = GRID_HEIGHT * GRID_CELL_SIZE + 1;
const GRID_COLOR_BACKGROUND: Color = Color::RGBA(22, 22, 22, 255);
const GRID_COLOR_LINE: Color = Color::RGBA(44, 44, 44, 255);
const GRID_COLOR_ENTITY: Color = Color::RGBA(255, 255, 255, 255);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = thread_rng();
    let entities = std::iter::repeat((GRID_WIDTH, GRID_HEIGHT))
        .map(|(x, y)| (rng.gen_range(0..x), rng.gen_range(0..y)))
        .unique()
        .take(15)
        .map(|(x, y)| (x * GRID_CELL_SIZE, y * GRID_CELL_SIZE))
        .map(|(x, y)| Rect::new(x, y, GRID_CELL_SIZE as u32, GRID_CELL_SIZE as u32))
        .collect::<Vec<Rect>>();

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

    let mut canvas = window.into_canvas().present_vsync().build()?;

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
        // draw the background
        canvas.set_draw_color(GRID_COLOR_BACKGROUND);
        canvas.clear();
        // draw the grid
        canvas.set_draw_color(GRID_COLOR_LINE);
        (0..(1 + GRID_WIDTH * GRID_CELL_SIZE))
            .step_by(GRID_CELL_SIZE as usize)
            .map(|x| canvas.draw_line((x, 0), (x, WINDOW_HEIGHT)))
            .collect::<Result<(), String>>()?;
        (0..(1 + GRID_HEIGHT * GRID_CELL_SIZE))
            .step_by(GRID_CELL_SIZE as usize)
            .map(|y| canvas.draw_line((0, y), (WINDOW_WIDTH, y)))
            .collect::<Result<(), String>>()?;

        canvas.set_draw_color(GRID_COLOR_ENTITY);

        canvas.fill_rects(&entities)?;
        entities
            .windows(2)
            .map(|p| (p[0].center(), p[1].center()))
            .map(|(a, b)| canvas.draw_line(a, b))
            .collect::<Result<(), String>>()?;
        canvas.present();
    }

    Ok(())
}

mod game;
mod grid;
mod commands;

extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::MouseState;
use std::time::Duration;

use game::Game;
use commands::key_mapper;
use grid::Grid;

const WINDOW: CellPoint<i32> = CellPoint { x:1400, y:800 };
const GRID: i32 = 2;
const CELL: CellPoint<i32> = CellPoint { 
    x: WINDOW.x / GRID,
    y: WINDOW.y / GRID, 
};
const LIMITS: CellPoint<i32> = CellPoint { x: 2, y: 100 };
const STEP: i32 = 2;

#[derive(Clone)]
struct CellPoint<T> { x: T, y: T, }

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Life", WINDOW.x as u32, WINDOW.y as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new();
    let mut mapper = key_mapper();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    while game.running() {
        let mouse = MouseState::new(&event_pump);
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(k), .. } => {
                    if let Some(action) = mapper.get(k) {
                        action(&mut game);
                    }
                },
                Event::Quit {..} => {
                    game.quit();
                    break;
                },
                _ => {},
            }
        }

        game.make_screen();
        display_board(&mut canvas, &game.screen, game.grid)?;
        game.place_cell(mouse.x(), mouse.y(), mouse.left());

        if game.paused() {
            game.next_gen();
            ::std::thread::sleep(Duration::from_millis(game.get_speed()));
        } 
    }
    Ok(())
}

fn display_board(canvas: &mut Canvas<Window>, cells: &Grid, grid: i32) -> Result<(), String> {
    canvas.clear();
    for y in 0..cells.height {
        for x in 0..cells.width {
            let color = if cells[y][x] { Color::RGB(0, 0, 0) } else { Color::RGB(255, 255, 255) };
            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(x as i32 * grid, y as i32 * grid, grid as u32, grid as u32))?; 
        }
    }
    canvas.present();
    Ok(())
}

fn alive(x: i32, y: i32, v: &Grid) -> bool {

    let n = cell_count(x as usize, y as usize, v);
    let curr = v[y as usize][x as usize] as i32;

    match (curr,  n) {
        (1, 0..=1) => false,
        (1, 4..=8) => false,
        (1, 2..=3) => true,
        (0, 3)     => true,
        (0, 0..=2) => false,
        (0, 4..=8) => false,
        _ => panic!("alive: error in match"),
    }
}

fn inc_x(n: usize) ->  usize {
    (n + 1) % CELL.x as usize
}

fn dec_x(n: usize) -> usize {
    if n == 0 { CELL.x as usize - 1 } else { (n - 1) as usize }
}

fn inc_y(n: usize) ->  usize {
    (n + 1) % CELL.y as usize
}

fn dec_y(n: usize) -> usize {
    if n == 0 { CELL.y as usize - 1 } else { n - 1 }
}

fn cell_count(x: usize, y: usize, v: &Grid) -> i32 {
    v[dec_y(y)][x] as i32 +
    v[inc_y(y)][x] as i32 +
    v[y][dec_x(x)] as i32 +
    v[y][inc_x(x)] as i32 +
    v[dec_y(y)][dec_x(x)] as i32 +
    v[dec_y(y)][inc_x(x)] as i32 +
    v[inc_y(y)][inc_x(x)] as i32 +
    v[inc_y(y)][dec_x(x)] as i32
}


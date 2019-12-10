extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::{MouseButton, MouseState};
 
const WIDTH: i32 = 900;
const HEIGHT: i32 = 900;
const GRID: i32 = 5;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Life", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let mut game_running = true;
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut cells = clear();
    let mut speed: u64 = 50;
    let mut block: bool = true;
    let white: Color = Color::RGB(255, 255, 255);


    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    'running: loop {
        let mouse = MouseState::new(&event_pump);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if speed > 10 {
                        speed -= 10;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    speed += 10;
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    cells = life_gen();
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    game_running = if game_running { false } else { true };
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    block = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    block = false;
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    cells = clear();
                    display_board(&mut canvas, &cells)?;
                },
                _ => {}
            }
        }

        if mouse.left() {
            game_running = false;
            let grid_x: usize = (mouse.x() / GRID) as usize;
            let grid_y: usize = (mouse.y() / GRID) as usize;
            cells[grid_y][grid_x] = block;
            display_board(&mut canvas, &cells)?;
        }

        if game_running {
            display_board(&mut canvas, &cells)?;
            cells = next_genoration(cells);
            ::std::thread::sleep(Duration::from_millis(speed));
        }
    }
    Ok(())
}

fn display_board(canvas: &mut Canvas<Window>, cells: &Vec<Vec<bool>>) -> Result<(), String> {
    canvas.clear();
    for (y, row) in cells.iter().enumerate() {
        for (x, &alive) in row.iter().enumerate() {
            if alive {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.fill_rect(Rect::new(x as i32 * GRID, y as i32 * GRID, GRID as u32, GRID as u32))?; 
            } else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(Rect::new(x as i32 * GRID, y as i32 * GRID, GRID as u32, GRID as u32))?; 
            }
        }
    }
    canvas.present();
    Ok(())
}


fn life_gen() -> Vec<Vec<bool>> {
    let mut v:Vec<Vec<bool>> = Vec::new();

    for y in  0..(HEIGHT / GRID) {
        v.push(Vec::new());
        for x in  0..(WIDTH / GRID) {
            v[y as usize].push(rand::random());
        }
    }
    v
}

fn clear() -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<bool>> = Vec::new();

    for y in 0..(HEIGHT / GRID) {
        v.push(Vec::new());
        for x in 0..(WIDTH / GRID) {
            v[y as usize].push(false);
        }
    }
    v
}

fn next_genoration(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut v2:Vec<Vec<bool>> = Vec::new();
    
    for y in 0..HEIGHT / GRID {
            v2.push(Vec::new());
        for x in 0..WIDTH / GRID{
            v2[y as usize].push(alive(x as i32,  y as i32, &v)); 
            }
        }
    v2
}

fn alive(x: i32, y: i32, v: &Vec<Vec<bool>>) -> bool {

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

fn inc(n: usize) ->  usize {
    (n + 1) % (WIDTH / GRID) as usize
}

fn dec(n: usize) -> usize {
    if n == 0 {
        ((WIDTH / GRID) - 1) as usize
    } else {
        (n - 1) as usize
    }
}

fn cell_count(x: usize, y: usize, v: &Vec<Vec<bool>>) -> i32 {
    v[dec(y)][x] as i32 +
    v[inc(y)][x] as i32 +
    v[y][dec(x)] as i32 +
    v[y][inc(x)] as i32 +
    v[dec(y)][dec(x)] as i32 +
    v[dec(y)][inc(x)] as i32 +
    v[inc(y)][inc(x)] as i32 +
    v[inc(y)][dec(x)] as i32
}


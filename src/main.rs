extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::MouseState;
 
const WIDTH: i32 = 900;
const HEIGHT: i32 = 900;
const GRID: i32 = 2;
const MAX: i32 = 2;
const MIN: i32 = 50;
const STEP: i32 = 2;

fn main() -> Result<(), String> {
    let mut grid: i32 = 20; 
    let mut width: i32 = WIDTH / grid;
    let mut height: i32 = HEIGHT / grid;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    
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
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if grid != MAX {
                        grid -= STEP;
                        width = WIDTH / grid;
                        height = HEIGHT / grid;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if grid != MIN {
                        grid += STEP;
                        width = WIDTH / grid;
                        height = HEIGHT / grid;
                    }
                },
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
                },
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    y = if y != 0 { y - 1 } else { y } 
                },
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    y = if y + height < HEIGHT / GRID { y + 1 } else { y };
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    x = if x + width < WIDTH / GRID { x + 1 } else { x };
                },
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    x = if x != 0 { x - 1 } else { x };
                },
                _ => {}
            }
        }

        let (x, y) = can_fit(x, y, width, height);
        let screen = get_screen(x, y, width, height, &cells);
        display_board(&mut canvas, &screen, grid)?;
        if mouse.left() {
            game_running = false;
            let screen_x: usize = (mouse.x() / grid) as usize;
            let screen_y: usize = (mouse.y() / grid) as usize;
            let (world_x, world_y) = screen_to_world_cords(x as usize, y as usize, screen_x, screen_y);
            cells[world_y][world_x] = block;
            display_board(&mut canvas, &screen, grid)?;
        }

        if game_running {
            cells = next_genoration(cells);
            ::std::thread::sleep(Duration::from_millis(speed));
        } 
    }
    Ok(())
}

fn display_board(canvas: &mut Canvas<Window>, cells: &Vec<Vec<bool>>, grid: i32) -> Result<(), String> {
    canvas.clear();
    for (y, row) in cells.iter().enumerate() {
        for (x, &alive) in row.iter().enumerate() {
            if alive {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.fill_rect(Rect::new(x as i32 * grid, y as i32 * grid, grid as u32, grid as u32))?; 
            } else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(Rect::new(x as i32 * grid, y as i32 * grid, grid as u32, grid as u32))?; 
            }
        }
    }
    canvas.present();
    Ok(())
}

fn get_screen(x: i32, y: i32, width: i32, height: i32, cells: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let x = x as usize;
    let y = y as usize;
    let width = width as usize;
    let height = height as usize;
    (0..height)
    .map(|dy|
        (0..width)
            .map(|dx| cells[y + dy][x + dx])
            .collect()
    )
    .collect()
}

fn screen_to_world_cords(x: usize, y: usize, sx: usize, sy: usize) -> (usize, usize) {
    let (x, y) = ((x + sx), (y + sy));
    (x, y)
}

fn can_fit(mut x: i32, mut y: i32, width: i32, height: i32) -> (i32, i32) {
    x = if x + width > WIDTH / GRID { WIDTH / GRID - width } else { x };
    y = if y + height > HEIGHT / GRID { HEIGHT / GRID - height } else { y };
    (x, y)
}

fn life_gen() -> Vec<Vec<bool>> {
    (0..HEIGHT / GRID)
    .map(|_|
        (0..WIDTH / GRID)
            .map(|_|rand::random())
            .collect::<Vec<bool>>()
    )
    .collect::<Vec<_>>()
}

fn clear() -> Vec<Vec<bool>> {
    (0..HEIGHT / GRID)
    .map(|_|
        (0..WIDTH / GRID)
            .map(|_|false)
            .collect::<Vec<bool>>()
    )
    .collect::<Vec<_>>()
}

fn next_genoration(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    (0..HEIGHT / GRID)
    .map(|y|
        (0..WIDTH / GRID)
            .map(|x|alive(x as i32, y as i32, &v))
            .collect::<Vec<bool>>()
    )
    .collect::<Vec<_>>()
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

fn inc_x(n: usize) ->  usize {
    (n + 1) % (WIDTH / GRID) as usize
}

fn dec_x(n: usize) -> usize {
    if n == 0 { ((WIDTH / GRID) - 1) as usize } else { (n - 1) as usize }
}

fn inc_y(n: usize) ->  usize {
    (n + 1) % (HEIGHT / GRID) as usize
}

fn dec_y(n: usize) -> usize {
    if n == 0 { ((HEIGHT / GRID) - 1) as usize } else { (n - 1) as usize }
}

fn cell_count(x: usize, y: usize, v: &Vec<Vec<bool>>) -> i32 {
    v[dec_y(y)][x] as i32 +
    v[inc_y(y)][x] as i32 +
    v[y][dec_x(x)] as i32 +
    v[y][inc_x(x)] as i32 +
    v[dec_y(y)][dec_x(x)] as i32 +
    v[dec_y(y)][inc_x(x)] as i32 +
    v[inc_y(y)][inc_x(x)] as i32 +
    v[inc_y(y)][dec_x(x)] as i32
}


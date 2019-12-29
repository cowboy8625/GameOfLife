extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse::MouseState;
use std::time::Duration;
use std::ops::{Index, IndexMut};
 
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
    let mut grid: i32 = 20; 
    let mut win: CellPoint<i32> = CellPoint { x: WINDOW.x / grid, y: WINDOW.y / grid };
    let mut loc: CellPoint<i32> = CellPoint { x: 0, y: 0 };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Life", WINDOW.x as u32, WINDOW.y as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let mut game_running = true;
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //let mut cells = clear();
    let mut cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, || false);


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
                    if grid != LIMITS.x {
                        grid -= STEP;
                        win.x = WINDOW.x / grid;
                        win.y = WINDOW.y / grid;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if grid != LIMITS.y {
                        grid += STEP;
                        win.x = WINDOW.x / grid;
                        win.y = WINDOW.y / grid;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, rand::random);
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
                    cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, || false);
                },
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    loc.y = if loc.y != 0 { loc.y - 1 } else { loc.y } 
                },
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    loc.y = if loc.y + win.x < CELL.y as i32 { loc.y + 1 } else { loc.y };
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    loc.x = if loc.x + win.x < CELL.x as i32 { loc.x + 1 } else { loc.x };
                },
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    loc.x = if loc.x != 0 { loc.x - 1 } else { loc.x };
                },
                _ => {}
            }
        }

        clamp(&mut loc, win.x, win.y);
        let screen = cells.sub_grid(loc.x as usize, loc.y as usize, win.x as usize, win.y as usize);
        display_board(&mut canvas, &screen, grid)?;
        if mouse.left() {
            game_running = false;
            let screen_x: usize = (mouse.x() / grid) as usize;
            let screen_y: usize = (mouse.y() / grid) as usize;
            let (world_x, world_y) = screen_to_world_cords(loc.x as usize, loc.y as usize, screen_x, screen_y);
            cells[world_y][world_x] = block;
        }

        if game_running {
            cells.next_gen();
            ::std::thread::sleep(Duration::from_millis(speed));
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

fn screen_to_world_cords(x: usize, y: usize, sx: usize, sy: usize) -> (usize, usize) {
    let (x, y) = ((x + sx), (y + sy));
    (x, y)
}

fn clamp(loc: &mut CellPoint<i32>, width: i32, height: i32) {
    loc.x = if loc.x + width > CELL.x as i32 { CELL.x - width } else { loc.x };
    loc.y = if loc.y + height > CELL.y as i32 { CELL.y - height } else { loc.y };
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


struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<bool>,
}

impl Grid {
    fn from_fn(width: usize, height: usize, f: impl FnMut() -> bool) -> Self {
        Self {
            width,
            height,
            tiles: std::iter::repeat_with(f).take(width * height).collect()
        }
    }

    fn next_gen(&mut self) {
        let r: &Grid = &self;
        self.tiles = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| alive(x as i32, y as i32, r)))
            .collect();
    }
    fn sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Self {
        let sub_tiles = (0..height)
            .flat_map(|dy|
                (0..width)
                    .map(move |dx| self[y + dy][x + dx])
            )
            .collect();

        Self {
            width,
            height,
            tiles: sub_tiles,
        }
    }
}

impl Index<usize> for Grid {
    type Output = [bool];

    fn index(&self, idx: usize) -> &Self::Output {
        let idx_start = idx * self.width;
        let idx_end = idx_start + self.width;

        self.tiles.index(idx_start..idx_end)
    }
}

impl IndexMut<usize> for Grid {

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        let idx_start = idx * self.width;
        let idx_end = idx_start + self.width;

        self.tiles.index_mut(idx_start..idx_end)
    }
}




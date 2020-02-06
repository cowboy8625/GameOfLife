extern crate rand;

use crate::{CELL, WINDOW, LIMITS, STEP, CellPoint};
use crate::grid::Grid;
pub struct Game {
    pub grid: i32,
    win: CellPoint<i32>,
    loc: CellPoint<i32>,
    speed: u64,
    block: bool,
    is_paused: bool,
    is_running: bool,
    cells: Grid,
    pub screen: Grid,
}

impl Game {
    pub fn new() -> Self {
        let cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, || false);
        let grid: i32 = 20; 
        let win: CellPoint<i32> = CellPoint { x: WINDOW.x / grid, y: WINDOW.y / grid };
        let loc: CellPoint<i32> = CellPoint { x: 0, y: 0 };
        let screen = cells.sub_grid(loc.x as usize, loc.y as usize, win.x as usize, win.y as usize);
        Self {
            grid,
            win,
            loc,
            speed: 50,
            block: true,
            is_paused: true,
            is_running: true,
            cells,
            screen,
        }
    }

    pub fn make_screen(&mut self) {
        self.clamp();
        self.screen = self.cells.sub_grid(self.loc.x as usize, self.loc.y as usize, self.win.x as usize,self. win.y as usize);
    }

    fn clamp(&mut self) {
        self.loc.x = if self.loc.x + self.win.x > CELL.x as i32 {
            CELL.x - self.win.x
        } else {
            self.loc.x
        };

        self.loc.y = if self.loc.y + self.win.y > CELL.y as i32 {
            CELL.y - self.win.y
        } else {
            self.loc.y
        };
    }

    pub fn start_stop(&mut self) {
        if self.is_paused { self.pause() } else { self.unpause() };
    }

    pub fn zoom_in(&mut self) {
        if self.grid != LIMITS.x {
            self.grid -= STEP;
            self.win.x = WINDOW.x / self.grid;
            self.win.y = WINDOW.y / self.grid;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.grid != LIMITS.y {
            self.grid += STEP;
            self.win.x = WINDOW.x / self.grid;
            self.win.y = WINDOW.y / self.grid;
        }
    }
}

// getters
impl Game {
    pub fn running(&self) -> bool {
        self.is_running
    }

    pub fn get_speed(&self) -> u64 {
        self.speed
    }

    pub fn paused(&self) -> bool {
        self.is_paused
    }
}

// setter
impl Game {
    pub fn place_cell(&mut self, x: i32, y: i32, is_pressed: bool) {
        if is_pressed {
            self.pause();
            let screen_x: usize = (x / self.grid) as usize;
            let screen_y: usize = (y / self.grid) as usize;
            let (world_x, world_y) = (self.loc.x as usize + screen_x, self.loc.y as usize + screen_y);
            self.cells[world_y][world_x] = self.block;
        }
    }

    pub fn next_gen(&mut self) {
        self.cells.next_gen();
    }

    pub fn pause(&mut self) {
        self.is_paused = false;
    }

    pub fn unpause(&mut self) {
        self.is_paused = true;
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn speed_up(&mut self) {
        if self.speed > 10 { self.speed -= 10; }
    }

    pub fn speed_down(&mut self) {
        if self.speed < 400 { self.speed += 10 }
    }

    pub fn spawn_random(&mut self) {
        self.cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, rand::random);
    }

    pub fn clear_board(&mut self) {
        self.cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, || false);
    }

    pub fn screen_up(&mut self) {
        self.loc.y = if self.loc.y != 0 { self.loc.y - 1 } else { self.loc.y } 
    }

    pub fn screen_down(&mut self) {
        self.loc.y = if self.loc.y + self.win.x < CELL.y as i32 { self.loc.y + 1 } else { self.loc.y };
    }

    pub fn screen_left(&mut self) {
        self.loc.x = if self.loc.x != 0 { self.loc.x - 1 } else { self.loc.x };
    }

    pub fn screen_right(&mut self) {
        self.loc.x = if self.loc.x + self.win.x < CELL.x as i32 { self.loc.x + 1 } else { self.loc.x };
    }

    pub fn swap_block(&mut self) {
        self.block = if self.block { false } else { true };
    }
}

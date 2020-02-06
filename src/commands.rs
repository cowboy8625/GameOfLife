use crate::game::Game;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;

type GameFunction = fn(&mut Game);
type KeyMaps = HashMap<Keycode, GameFunction>;

pub struct Mapper {
    key_maps: KeyMaps
}

impl Mapper {
    pub fn new() -> Self {
        Self { key_maps: HashMap::new() }
    }

    pub fn add_key(&mut self, key: Keycode, event: fn(&mut Game)) {
        self.key_maps.insert(key, event);
    }

    pub fn get(&mut self, key: Keycode) -> Option<&fn(&mut Game)> {
        self.key_maps.get(&key)
    }
}
/*
Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    break 'running
},
*/
pub fn key_mapper() -> Mapper {
    let mut map = Mapper::new();

    map.add_key(Keycode::Escape, stop);
    map.add_key(Keycode::Up, speed_up);
    map.add_key(Keycode::Down, speed_down);
    map.add_key(Keycode::R, spawn_random);
    map.add_key(Keycode::C, clear_board);
    map.add_key(Keycode::S, start_stop);
    map.add_key(Keycode::K, screen_up);
    map.add_key(Keycode::J, screen_down);
    map.add_key(Keycode::H, screen_left);
    map.add_key(Keycode::L, screen_right);
    map.add_key(Keycode::Left, zoom_out);
    map.add_key(Keycode::Right, zoom_in);
    map.add_key(Keycode::Num1, swap_block);

    map
}

fn stop(game: &mut Game) {
    game.quit();
}

fn speed_up(game: &mut Game) {
    game.speed_up();
}

fn speed_down(game: &mut Game) {
    game.speed_down();
}

fn spawn_random(game: &mut Game) {
    game.spawn_random();
}

fn clear_board(game: &mut Game) {
    game.clear_board();
}

fn start_stop(game: &mut Game) {
    game.start_stop();
}

fn screen_up(game: &mut Game) {
    game.screen_up();
}

fn screen_down(game: &mut Game) {
    game.screen_down();
}

fn screen_left(game: &mut Game) {
    game.screen_left();
}

fn screen_right(game: &mut Game) {
    game.screen_right();
}

fn zoom_in(game: &mut Game) {
    game.zoom_in();
}

fn zoom_out(game: &mut Game) {
    game.zoom_out();
}

fn swap_block(game: &mut Game) {
    game.swap_block();
}

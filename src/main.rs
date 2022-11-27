use std::vec::Vec;
use minifb::{Key, Window, WindowOptions};

mod automaton;
use crate::automaton::Rules;
mod game_of_life;
use crate::game_of_life::GameOfLifeAutomaton;

const M: usize = 80;
const N: usize = 120;

const CELL_SIZE: usize = 10;

const WIDTH: usize = N * CELL_SIZE;
const HEIGHT: usize = M * CELL_SIZE;


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new(
        "Cellular automaton - ESC to quit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let cells = vec![vec![0u32; WIDTH]; HEIGHT];
    let mut automaton = GameOfLifeAutomaton::new(M, N, cells);
    automaton.init_rand();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        automaton.next();

        for (index, cell) in buffer.iter_mut().enumerate() {
            let x = index / WIDTH;
            let y = index % WIDTH;

            let i = x / CELL_SIZE;
            let j = y / CELL_SIZE;

            *cell = match automaton.get_cell(i, j) {
                0 => 0xFFFFFF,
                1 => 0,
                _ => 0
            };
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

use std::vec::Vec;
use minifb::{Key, Window, WindowOptions};

mod automaton;
mod game_of_life;
mod wildfire;
mod color_gradient;

use crate::automaton::{Access, Rules, Init};
use crate::game_of_life::GameOfLifeAutomaton;
use crate::wildfire::WildfireAutomaton;
use crate::color_gradient::ColorGradientAutomaton;

const M: usize = 400;
const N: usize = 600;

const CELL_SIZE: usize = 2;

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
    window.limit_update_rate(Some(std::time::Duration::from_micros(3 * 16600)));

    let cells = vec![vec![0u32; WIDTH]; HEIGHT];
    let mut automaton = ColorGradientAutomaton::new(M, N, true, cells);
    automaton.init_rand();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        automaton.next();

        for (index, cell) in buffer.iter_mut().enumerate() {
            let x = index / WIDTH;
            let y = index % WIDTH;

            let i = x / CELL_SIZE;
            let j = y / CELL_SIZE;

            *cell = automaton.get_cell_color(i, j);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

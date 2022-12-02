use std::vec::Vec;
use minifb::{Key, Window, WindowOptions};


mod automaton_1d;
mod automaton_2d;
mod game_of_life;
mod color_gradient;
mod wildfire;
mod epidemic;


fn main() {
    let m = 200;
    let n = 300;
    let cell_size = 4;
    let width = n * cell_size;
    let height = m * cell_size;

    let mut buffer: Vec<u32> = vec![0; width * height];
    
    let mut window = Window::new(
        "Cellular automaton - ESC to quit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(2 * 16600)));

    let mut automaton = epidemic::new(m, n, true);
    automaton.init_rand(true);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        automaton.next();

        for (index, cell) in buffer.iter_mut().enumerate() {
            let x = index / width;
            let y = index % width;

            let i = x / cell_size;
            let j = y / cell_size;

            *cell = automaton.get_cell_color(i, j);
        }
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

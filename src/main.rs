use std::vec::Vec;
use automaton_1d::{generate_image};
use minifb::{Key, Window, WindowOptions};


mod automaton_1d;
mod automaton_2d;
mod elementary;
mod game_of_life;
mod color_gradient;
mod wildfire;
mod epidemic;


fn main() {
    let m = 20000;
    let n = 1000;
    let cell_size = 1;
    let width = n * cell_size;
    let height = m * cell_size;

    let mut a = elementary::new(n, &22, false);
    let img = generate_image(width, height, cell_size, &mut a);
    img.save("rule_22_4.png").unwrap();
    // let mut buffer: Vec<u32> = vec![0; width * height];
    // 
    // let mut window = Window::new(
    //     "Cellular automaton - ESC to quit",
    //     width,
    //     height,
    //     WindowOptions::default(),
    // )
    // .unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
// 
    // let mut automaton = elementary::new(n, &90, false);
    // let mut k = 0;
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     automaton_1d::update_buffer(&mut buffer, &automaton, k, width, cell_size);
    //     window.update_with_buffer(&buffer, width, height).unwrap();
    //     if k < m - 1 {
    //         automaton.next();
    //         k += 1;
    //     }
    // }
}

mod automaton_1d;
mod automaton_2d;
mod color;

use automaton_1d::{generate_image, elementary};
// use minifb::{Key, Window, WindowOptions};

fn main() {
    const cell_size: usize = 1;
    const rules: [u8; 5] = [22, 30, 45, 73, 99];

    for i in 0..rules.len() {
        for torus in [false, true] {
            for (m, n) in [(400, 400),
                                         (400, 600),
                                         (600, 400),
                                         (4000, 4000),
                                         (4000, 6000),
                                         (6000, 4000)] {
                let width = n * cell_size;
                let height = m * cell_size;
                let mut a = elementary::new(n, &rules[i], torus);
                let img = generate_image(width, height, cell_size, &mut a);
                img.save(format!(
                    "examples/rule{}/rule{}_w{}_h{}_cs{}_torus{}.png",
                    &rules[i],
                    &rules[i],
                    width,
                    height,
                    cell_size,
                    torus
                )).unwrap();
            }
        }
    }

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

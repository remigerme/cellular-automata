use crate::automaton_2d::Automaton2D;


fn next_state(a: &Automaton2D<u8>, i: usize, j: usize) -> u8 {
    let neighbours = a.get_moore_neighbours(i, j);
    let nb_neighbours_alive = neighbours.iter().fold(0, |a, b| a + b);

    match a.get_cell(i, j) {
        0 => if nb_neighbours_alive == 3 {1} else {0},
        1 => if nb_neighbours_alive == 2 || nb_neighbours_alive == 3 {1} else {0},
        _ => 0
    }
}


pub fn new(m: usize, n: usize, torus: bool) -> Automaton2D<u8> {
    Automaton2D::new(
        m,
        n,
        2,
        vec![0, 1],
        torus,
        Box::new(next_state),
        Box::new(|x| match x {
            0 => 0xFFFFFF,
            1 => 0,
            _ => panic!("Unknown cell state")
        }),
        vec![vec![0; n]; m]
    )
}

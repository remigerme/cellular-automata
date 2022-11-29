use crate::automaton::Automaton;


fn next_state(a: &Automaton, i: usize, j: usize) -> u32 {
    let neighbours = a.get_neighbours(i, j);
    let nb_neighbours_alive = neighbours.iter().fold(0, |a, b| a + b);

    match a.get_cell(i, j) {
        0 => if nb_neighbours_alive == 3 {1} else {0},
        1 => if nb_neighbours_alive == 2 || nb_neighbours_alive == 3 {1} else {0},
        _ => 0
    }
}


pub fn new(m: usize, n: usize, torus: bool) -> Automaton {
    Automaton::new(
        m,
        n,
        2,
        torus,
        Box::new(next_state),
        vec![0xFFFFFF, 0],
        vec![vec![0; n]; m]
    )
}

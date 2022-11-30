use std::collections::HashMap;
use rand::Rng;
use crate::automaton::Automaton;


const ROCK: u8 = 0;
const GREY: u32 = 0xB0B0B0;
const ASHES: u8 = 1;
const BLACK: u32 = 0x3B3737;
const YOUNG: u8 = 2;
const GREEN: u32 = 0x68E180;
const OLD: u8 = 3;
const DARK_GREEN: u32 = 0x24A609;
const NEW_FIRE: u8 = 4;
const YELLOW: u32 = 0xF2F26E;
const FIRE: u8 = 5;
const ORANGE: u32 = 0xE48F01;
const FADING_FIRE: u8 = 6;
const RED: u32 = 0xBA2E0B;


fn next_state(a: &Automaton<u8>, i: usize, j: usize) -> u8 {
    let mut rng = rand::thread_rng();
    match a.get_cell(i, j) {
        ROCK => ROCK,
        ASHES => if rng.gen::<f64>() < 0.001 {YOUNG} else {ASHES},
        YOUNG => {
            let neighbours = a.get_moore_neighbours(i, j);
            if rng.gen::<f64>() < 0.01 && neighbours.contains(&NEW_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.02 && neighbours.contains(&FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.01 && neighbours.contains(&FADING_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.005 {
                OLD
            } else {
                YOUNG
            }
        },
        OLD => {
            let neighbours = a.get_moore_neighbours(i, j);
            if rng.gen::<f64>() < 0.1 && neighbours.contains(&NEW_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.2 && neighbours.contains(&FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.1 && neighbours.contains(&FADING_FIRE) {
                NEW_FIRE
            } else if rng.gen::<f64>() < 0.00005 && neighbours.iter().filter(|&n| *n == OLD).count() >= 5 {
                NEW_FIRE
            } else {
                OLD
            }
        },
        NEW_FIRE => if rng.gen::<f64>() < 0.1 {FIRE} else {NEW_FIRE},
        FIRE => if rng.gen::<f64>() < 0.1 {FADING_FIRE} else {FIRE},
        FADING_FIRE => if rng.gen::<f64>() < 0.1 {ASHES} else {FADING_FIRE},
        _ => ROCK,
    }
}


pub fn new(m: usize, n: usize, torus: bool) -> Automaton<u8> {
    Automaton::new(
        m,
        n,
        7,
        vec![ROCK, ASHES, YOUNG, OLD, NEW_FIRE, FIRE, FADING_FIRE],
        torus,
        Box::new(next_state),
        HashMap::from([
            (ROCK, GREY),
            (ASHES, BLACK),
            (YOUNG, GREEN),
            (OLD, DARK_GREEN),
            (NEW_FIRE, YELLOW),
            (FIRE, ORANGE),
            (FADING_FIRE, RED)
        ]),
        vec![vec![ROCK; n]; m] 
    )
}

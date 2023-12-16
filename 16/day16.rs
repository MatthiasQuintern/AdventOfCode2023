use std::io::{self, BufRead};
use std::collections::VecDeque;

const UP: u8    = 0b00001000;
const DOWN: u8  = 0b00000100;
const RIGHT: u8 = 0b00000010;
const LEFT: u8  = 0b00000001;
const DIRECTION: u8 = UP | DOWN | LEFT | RIGHT;

const PIPE: u8  = 0b10000000;
const DASH: u8  = 0b01000000;
const SLASH: u8 = 0b00100000;
const BSLASH: u8= 0b00010000;
const NOTHING: u8= 0;
const TILE: u8 = PIPE | DASH | SLASH | BSLASH;

struct Vec2D<T> {
    data: Vec<T>,
    xlen: usize,
    ylen: usize,
}

impl<T: std::clone::Clone> Vec2D::<T> {
    fn new(xlen: usize, ylen: usize, t: T ) -> Vec2D::<T> {
        let vec2_d = Vec2D::<T>{ data: vec![t.clone(); xlen * ylen], xlen, ylen };
        return vec2_d;
    }
    fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if y < self.ylen && x < self.xlen { return Some(&mut self.data[x + y * self.xlen]) }
        else { return None }
    }
    fn at(&self, x: usize, y: usize) -> Option<&T> {
        if y < self.ylen && x < self.xlen { return Some(&self.data[x + y * self.xlen]) }
        else { return None }
    }

}

impl std::fmt::Display for Vec2D::<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                let c = *self.at(x, y).unwrap();
                let _ = write!(f, "{}", match c {
                    PIPE => '|',
                    DASH => '-',
                    SLASH => '/',
                    BSLASH => '\\',
                    _ => if c & DIRECTION != 0 { '#' } else { '.' },
                });
            }
            let _ = writeln!(f, "");
        }

        writeln!(f, "")
    }
}

fn go_direction(x: usize, y: usize, x_max: usize, y_max: usize, direction: u8) -> Option<(usize, usize)> {
    match direction {
        DOWN  => if y < y_max - 1 { return Some((x, y + 1)) },
        UP    => if y > 0 { return Some((x, y - 1)) },
        RIGHT => if x < x_max - 1 { return Some((x + 1, y)) },
        LEFT  => if x > 0  { return Some((x - 1, y)) },
        _ => panic!("Invalid direction: '{}'", direction)
    }
    None
}



fn travel_beam(contraption: &mut Vec2D::<u8>, beam_start: (usize, usize, u8)) -> usize {
    let mut n_travelled: usize = 0;
    let mut todo_beams = VecDeque::from([beam_start]);
    while !todo_beams.is_empty() {
        let (mut x, mut y, mut direction) = todo_beams.pop_front().unwrap();

        // println!("New start: x={:3}, y={:3}, d={:04b}", x, y, direction);
        'inner: loop {
            // x , y = current tile, direction is direction of previous tile
            // println!("x={:3}, y={:3}, d={:04b}", x, y, direction);
            let tile = contraption.at_mut(x, y).unwrap();
            if *tile & DIRECTION == 0 { n_travelled += 1 }
            if *tile & direction != 0 { 
                // println!("Tile has been reached from this direction before");
                break 'inner;
            } // has been traversed in this direction before
            *tile |= direction;
            match *tile & TILE {
                PIPE => {
                    if direction & (LEFT | RIGHT) != 0 {
                        todo_beams.push_back((x, y, UP));
                        direction = DOWN;
                    }
                }
                DASH => {
                    if direction & (UP | DOWN) != 0 {
                        // todo_beams.push_back((x, y, LEFT));
                        // direction = RIGHT;
                        todo_beams.push_back((x, y, RIGHT));
                        direction = LEFT;
                    }
                }
                SLASH => {
                    if direction & (RIGHT | UP) != 0 { direction ^= RIGHT | UP }
                    else { direction ^= LEFT | DOWN }
                },
                BSLASH => {
                    if direction & (LEFT | UP) != 0 { direction ^= LEFT | UP }
                    else { direction ^= RIGHT | DOWN }
                },
                _ => {}
            }
            // println!(" -> x={:3}, y={:3}, d={:04b}", x, y, direction);
            if let Some(t) = go_direction(x, y, contraption.xlen, contraption.ylen, direction) {
                (x, y) = t;
            }
            else {  // out of bounds
                // println!("Travelled out of bounds");
                break 'inner;
            }
        }
    }
    return n_travelled;

}


// fn count_traversed_tiles(contraption: &mut Vec2D::<u8>) -> usize {
//     let mut n_tiles: usize = 0;
//     for x in 0..contraption.xlen {
//         for y in 0..contraption.ylen {
//             if *contraption.at(x, y).unwrap() & DIRECTION != 0 { n_tiles += 1 }
//         }
//     }
//     return n_tiles;
// }

fn main() {
    let input = "input.txt";
    let mut lines = read_lines(&input);
    let line_length = lines.next().expect("No line").unwrap().len();
    let n_lines = lines.count() + 1;  // already consumed one
    lines = read_lines(&input);
    let mut contraption = Vec2D::<u8>::new(line_length, n_lines, 0);
    for (y, line) in lines.map(|r| r.ok().unwrap()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            *contraption.at_mut(x, y).unwrap() = match c {
                '|' => PIPE,
                '-' => DASH,
                '/' => SLASH,
                '\\' => BSLASH,
                _ => NOTHING
            }
        }
    }
    let mut n_tiles = travel_beam(&mut contraption, (0, 0, RIGHT));
    println!("Beamed tiles: (1): {}", n_tiles);

    let reset = |c: &mut Vec2D<u8>| c.data.iter_mut().for_each(|v| *v &= !DIRECTION) ;

    // it is very stupid at no real need to optimize at this speed
    for y in 0..contraption.ylen {
        n_tiles = n_tiles.max(travel_beam(&mut contraption, (0, y, RIGHT)));
        reset(&mut contraption);
        n_tiles = n_tiles.max(travel_beam(&mut contraption, (line_length - 1, y, LEFT)));
        reset(&mut contraption);
    }
    for x in 0..contraption.xlen {
        n_tiles = n_tiles.max(travel_beam(&mut contraption, (x, 0, DOWN)));
        reset(&mut contraption);
        n_tiles = n_tiles.max(travel_beam(&mut contraption, (x, n_lines - 1, UP)));
        reset(&mut contraption);
    }
    println!("Max beamed tiles: (2): {}", n_tiles);
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

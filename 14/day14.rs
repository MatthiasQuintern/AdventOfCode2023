use std::io::{self, BufRead};

type Rocks = Vec<Vec<u8>>;

// tilt so that all 'O' characters want to DECREMENT their Y index
// using custom `at` function that "remaps" x and y, this function can be used for all directions
fn tilt<F>(rocks: &mut Rocks, at: F, x_len: usize, y_len: usize) 
where F: Fn(&mut Rocks, usize, usize) -> &mut u8 {
    for x in 0..x_len {
        let mut next_y: usize = 0;  // lowest reachable position
        for y in 0..y_len {
            match *at(rocks, x, y) as char {
                '#' => next_y = y + 1,
                'O' => {
                    if next_y != y {
                        *at(rocks, x, next_y) = 'O' as u8;
                        *at(rocks, x, y) = '.' as u8;
                    }
                    while next_y < y_len - 1 {
                        next_y += 1;
                        if *at(rocks, x, next_y) != '#' as u8 { break }
                    }
                },
                _ => {},
            }
        }
    }
}

//   --> y
// |
// V
// x
// West is the orientation that looks like the input
// When printing with one of these, the rocks want to roll to the left side
fn west_at(rocks: &mut Rocks, x: usize, y: usize) -> &mut u8 { &mut rocks[x][y] }
fn east_at(rocks: &mut Rocks, x: usize, y: usize) -> &mut u8 { let ylen = rocks[0].len(); &mut rocks[x][ylen - y - 1] }
fn south_at (rocks: &mut Rocks, x: usize, y: usize) -> &mut u8 { let xlen = rocks.len(); let ylen = rocks[0].len(); &mut rocks[xlen - y - 1][ylen - x - 1] }
fn north_at (rocks: &mut Rocks, x: usize, y: usize) -> &mut u8 { let ylen = rocks[0].len(); &mut rocks[y][ylen - x - 1] }

fn print<F>(rocks: &mut Rocks, at: F, x_len: usize, y_len: usize) 
where F: Fn(&mut Rocks, usize, usize) -> &mut u8 {
    (0..x_len).for_each(|x| { (0..y_len).for_each(|y| print!("{}", *at(rocks, x, y) as char)); println!() });
}
fn weigh<F>(rocks: &mut Rocks, at: F, x_len: usize, y_len: usize) -> usize
where F: Fn(&mut Rocks, usize, usize) -> &mut u8 {
    let mut weight: usize = 0;
    (0..x_len).for_each(|x| (0..y_len).for_each(|y| if *at(rocks, x, y) == 'O' as u8 { weight += y_len - y }) );
    return weight;
}

fn main() {
    // let input = "example.txt";
    let input = "input.txt";
    let lines = read_lines(&input);
    let mut original: Rocks = Vec::new();
    for line in lines.map(|r| r.ok().unwrap()) {
        original.push(line.chars().map(|c| c as u8).collect());
    }
    let ew_xlen = original.len();
    let ew_ylen = original[0].len();
    let ns_xlen = ew_ylen;
    let ns_ylen = ew_xlen;

    // TASK 1
    let mut rocks = original.clone();
    tilt(&mut rocks, north_at, ns_xlen, ns_ylen);
    println!("Load on north support beams (1): {}", weigh(&mut rocks, north_at, ns_xlen, ns_ylen));
    // print(&mut rocks, west_at, ew_xlen, ew_ylen);

    // TASK 2
    rocks = original.clone();
    // find a cycle
    let mut rocks_store: Vec<Rocks> = Vec::new();
    let mut cycle_start: usize = 0;
    let mut cycle_length: usize = 0;
    for i in 0..1_000_000_000 as usize {
        tilt(&mut rocks, north_at, ns_xlen, ns_ylen);
        tilt(&mut rocks, west_at, ew_xlen, ew_ylen);
        tilt(&mut rocks, south_at, ns_xlen, ns_ylen);
        tilt(&mut rocks, east_at, ew_xlen, ew_ylen);
        if let Some(j) = rocks_store.iter().position(|r| r.eq(&rocks)) {
            cycle_start = j;
            cycle_length = i - j;
            println!("Cycle found! rocks at {} == rocks at {}", i, j);
            break;
        }
        rocks_store.push(rocks.clone());
    }
    // do the cycles that remain after the ... Cycle
    for _ in 0..(1_000_000_000 as usize - cycle_start) % cycle_length - 1 {
        tilt(&mut rocks, north_at, ns_xlen, ns_ylen);
        tilt(&mut rocks, west_at, ew_xlen, ew_ylen);
        tilt(&mut rocks, south_at, ns_xlen, ns_ylen);
        tilt(&mut rocks, east_at, ew_xlen, ew_ylen);
    }
    // tilt(&mut rocks, north_at, ns_xlen, ns_ylen);
    println!("Load on north support beams (2): {}", weigh(&mut rocks, north_at, ns_xlen, ns_ylen));
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

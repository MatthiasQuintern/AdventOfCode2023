use std::io::{self, BufRead};

// connection2 = connection12 ^ connection1
const N : u8    = 0b00001000;
const S: u8     = 0b00000100;
const E: u8     = 0b00000010;
const W: u8     = 0b00000001;
const START: u8 = 0b00010000;
const NOTHING: u8 = 0b00000000;
const LOOP_TILE : u8 = 0b10000000;

#[derive(Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn n(&self) -> Option<Pos> {
        if self.y > 0 { return Some(Pos{x: self.x, y: self.y-1}) }
        else { return None }
    }
    fn s(&self) -> Option<Pos> { Some(Pos{x: self.x, y: self.y+1}) }
    fn w(&self) -> Option<Pos> {
        if self.x > 0 { return Some(Pos{x: self.x-1, y: self.y}) }
        else { return None }
    }
    fn e(&self) -> Option<Pos> { Some(Pos{x: self.x+1, y: self.y}) }
}
impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn follow(tiles: &mut Vec<Vec<u8>>, start: &Pos, mut direction: u8) -> Option<usize> {
    let xlen: usize = tiles[0].len();  // assume all are same size
    let ylen: usize = tiles.len();
    let mut steps: usize = 0;
    let mut current = *start;
    // mark the start tile as loop tile and with its directions
    tiles[current.y][current.x] = START | direction | LOOP_TILE;
    loop {
        // println!("Now at {}={} moving in direction {:08b}", current, tiles[current.y][current.x], direction);
        let next: Option<Pos> = match direction {
            N => current.n(),
            S => current.s(),
            E => current.e(),
            W => current.w(),
            _ => panic!("Invalid direction {}", direction),
        };
        if next.is_none() { println!("BOUND"); return None }  // reached bounds
        current = next.unwrap();
        steps += 1;
        // get opposite direction 
        let opposite: u8;
        if direction & 0b1100 != 0 { opposite = direction ^ 0b1100 }
        else { opposite = direction ^0b0011 }
        // check if back at start
        // println!("Tile={:08b}", tiles[current.y][current.x]);
        if tiles[current.y][current.x] & START != 0 {
            tiles[current.y][current.x] |= opposite;
            return Some(steps) 
        }
        // check if connected to last tile
        if tiles[current.y][current.x] & opposite == 0  { println!("NC"); return None }  // not connected to last
        direction = tiles[current.y][current.x] ^ opposite;
        // if direction == 0 { return None }  // dead end
        // mark as part of loop
        tiles[current.y][current.x] |= LOOP_TILE;
    }
}


fn get_inner_area(tiles: &Vec<Vec<u8>>) -> usize {
    let mut count: usize = 0;
    for y in 0..tiles.len() {
        let mut in_loop: bool = false;
        let mut last_corner_tile: u8 = 0;
        for x in 0..tiles[0].len() {
            if tiles[y][x] & LOOP_TILE != 0 {  // loop tile
                if tiles[y][x] & (E | W) == 0 {  // vertical tile
                    in_loop = !in_loop 
                }
                else if tiles[y][x] & (N | S) != 0 {  // corner
                    // println!("Corner tile {:08b}", tiles[y][x]);
                    if last_corner_tile == 0 { 
                        last_corner_tile = tiles[y][x]; 
                    }
                    // toggle if vertical direction of corner tile differs
                    else {
                        if (last_corner_tile & 0b1100) != (tiles[y][x] & 0b1100) { in_loop = !in_loop }
                        last_corner_tile = 0;
                    }
                }
            }
            else if in_loop {
                count += 1;
                println!("({},{})", x, y);
            }
        }
    }
    return count;
}

fn main() {
    // let input = "example.txt";
    let input = "input.txt";
    let lines = read_lines(&input);
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    tiles.reserve(150);
    let mut start_pos = Pos{x: 0, y: 0};
    for (y, line) in lines.map(|r| r.ok().unwrap()).enumerate() {
        tiles.push(line.chars().enumerate().map(|(x, c)| {
            match c {
                '|' => N | S,
                '-' => E | W,
                'L' => N | E,
                'J' => N | W,
                '7' => S | W,
                'F' => S | E,
                'S' => { start_pos = Pos{x, y}; START },
                _ => NOTHING
            }
        }).collect());
    }
    let mut distance: usize = 0;
    let mut num_tiles: usize = 0;
    for direction in [N, S, E, W] {
        println!("Starting at ({},{})", start_pos.x, start_pos.y);
        if let Some(steps) = follow(&mut tiles, &start_pos, direction) {
            assert!(steps % 2 == 0, "steps={} not even", steps);
            num_tiles = steps;
            distance = steps / 2;
            break;
        }
        else {  // reset visited tiles
            for y in 0..tiles.len() {
                for x in 0..tiles[0].len() {
                    tiles[y][x] &= 0b0111_1111;
                }
            }
        }
    }
    println!("Furthest tile distance: (1): {}", distance);
    // unmark the start tile
    tiles[start_pos.y][start_pos.x] &= !START;
    let area = get_inner_area(&tiles);
    println!("Area in loop: (2): {}", area);

}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

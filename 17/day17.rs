use std::io::{self, BufRead};
use std::collections::VecDeque;

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
}
impl<T> Vec2D::<T> {
    fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if y < self.ylen && x < self.xlen { return Some(&mut self.data[x + y * self.xlen]) }
        else { return None }
    }
    fn at(&self, x: usize, y: usize) -> Option<&T> {
        if y < self.ylen && x < self.xlen { return Some(&self.data[x + y * self.xlen]) }
        else { return None }
    }
}
impl<T> std::fmt::Display for Vec2D::<T>
    where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                let _ = write!(f, "{}", *self.at(x, y).unwrap());
            }
            let _ = writeln!(f, "");
        }
        writeln!(f, "")
    }
}


// const UP: u8    = 0b00001000;
// const DOWN: u8  = 0b00000100;
// const RIGHT: u8 = 0b00000010;
// const LEFT: u8  = 0b00000001;
const UP: u8    = 0;
const DOWN: u8  = 1;
const RIGHT: u8 = 2;
const LEFT: u8  = 3;

#[derive(Clone, Copy)]
struct Node {
    heat_loss: u8,
    min_heat_loss: u32,
    steps_from: [u8; 4],
    // direction_from: u8,
    // steps_from: u8,
    updated: bool,  // if false, skip when queued
 }
impl Node {
    fn new() -> Node { return Node{heat_loss: 0, min_heat_loss: u32::MAX, steps_from: [0; 4], updated: true, } }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[l={}, ml={:04}{}]", self.heat_loss, self.min_heat_loss, match self.direction_from {
            // UP => '^', DOWN => 'v', LEFT => '<', RIGHT => '>', _ => 'X'})
            UP => 'v', DOWN => '^', LEFT => '>', RIGHT => '<', _ => 'X'})
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

fn traverse_grapgh(graph: &mut Vec2D::<Node>) -> u32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);

    let xlen= graph.xlen;
    let ylen = graph.ylen;
    
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let opposite: u8;
        let node = *graph.at(x ,y).unwrap();  // copy required because we borrow from graph later :(
        if !node.updated { continue; }
        // update surrounding reachable nodes if they can be reached at a lower cost
        // dont check the one from where we came from
        // if updated, queue them
        if node.direction_from & (UP | DOWN) != 0 { opposite = node.direction_from ^ (UP | DOWN) }
        else { opposite = node.direction_from ^ (LEFT | RIGHT) }

        let mut update_direction = |direction: u8| {
            if direction == opposite { return }  // dont update the direction from which this node was reached
            let mut steps: u8 = 1;
            if node.direction_from == direction {
                if node.steps_from >= 3 { return }  // unreachable, because we cant walk more than 3 blocks in one direction
                steps += node.steps_from;
            }
            if let Some((other_x, other_y)) = go_direction(x, y, xlen, ylen, direction) {
                let other_node = graph.at_mut(other_x, other_y).unwrap();
                let heat_loss = node.min_heat_loss + other_node.heat_loss as u32;
                // TODO use array
                if other_node.min_heat_loss > heat_loss {  // update all
                    other_node.min_heat_loss = node.min_heat_loss + other_node.heat_loss as u32;
                    other_node.direction_from = direction;
                    other_node.steps_from = steps;
                    other_node.updated = true;
                    queue.push_back((other_x, other_y));
                }
                else if other_node.min_heat_loss == heat_loss {  // mark additional possible direction
                    other_node.min_heat_loss = node.min_heat_loss + other_node.heat_loss as u32;
                    other_node.direction_from = direction;
                    other_node.steps_from = steps;
                    other_node.updated = true;
                    queue.push_back((other_x, other_y));
                }
            }
        };
        update_direction(UP);
        update_direction(DOWN);
        update_direction(LEFT);
        update_direction(RIGHT);
        graph.at_mut(x ,y).unwrap().updated = false;
    }
    

    let min_heat_loss = graph.at(xlen - 1, ylen - 1).unwrap().min_heat_loss;

    return min_heat_loss;
}




fn main() {
    // let input = "input.txt";
    let input = "example.txt";
    // let input = "example2.txt";
    let mut lines = read_lines(&input);
    let line_length = lines.next().expect("No line").unwrap().len();
    let n_lines = lines.count() + 1;  // already consumed one
    lines = read_lines(&input);
    let mut city_blocks = Vec2D::<Node>::new(line_length, n_lines, Node::new());
    for (y, line) in lines.map(|r| r.ok().unwrap()).enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            city_blocks.at_mut(x, y).unwrap().heat_loss = c - b'0';
        }
    }
    city_blocks.at_mut(0, 0).unwrap().min_heat_loss = 0;
    println!("{}", city_blocks);
    let min_heat_loss = traverse_grapgh(&mut city_blocks);

    println!("{}", city_blocks);

    println!("Minimum heat loss: (1): {}", min_heat_loss);

    // let reset = |c: &mut Vec2D<u8>| c.data.iter_mut().for_each(|v| *v &= !DIRECTION) ;

    // // it is very stupid at no real need to optimize at this speed
    // for y in 0..contraption.ylen {
    //     n_tiles = n_tiles.max(travel_beam(&mut contraption, (0, y, RIGHT)));
    //     reset(&mut contraption);
    //     n_tiles = n_tiles.max(travel_beam(&mut contraption, (line_length - 1, y, LEFT)));
    //     reset(&mut contraption);
    // }
    // for x in 0..contraption.xlen {
    //     n_tiles = n_tiles.max(travel_beam(&mut contraption, (x, 0, DOWN)));
    //     reset(&mut contraption);
    //     n_tiles = n_tiles.max(travel_beam(&mut contraption, (x, n_lines - 1, UP)));
    //     reset(&mut contraption);
    // }
    // println!("Max beamed tiles: (2): {}", n_tiles);
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}


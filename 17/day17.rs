use std::io::{self, BufRead};
use std::collections::VecDeque;

#[derive(Clone)]
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


type Direction   = u8;
const UP: Direction    = 0;
const DOWN: Direction  = 1;
const RIGHT: Direction = 2;
const LEFT: Direction  = 3;

const OPPOSITE: [Direction; 4] = [DOWN, UP, LEFT, RIGHT];
const CHAR: [char; 4] = ['^', 'v', '>', '<'];

type HeatLoss = u32;
#[derive(Clone, Copy)]
struct Node {
    heat_loss: HeatLoss,
    min_heat_from: [HeatLoss; 4],
    steps_from: [u8; 4],
    updated_from: [bool; 4],  // if false, skip when queued
 }
impl Node {
    fn new() -> Node { return Node{heat_loss: 0, min_heat_from: [u32::MAX; 4], steps_from: [0; 4], updated_from: [true; 4] } }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _  = write!(f, "[l={}", self.heat_loss);
        for i in 0..4 {
            if self.min_heat_from[i] == u32::MAX { 
                let _ = write!(f, "|    ");
            }
            else {
                let _ = write!(f, "|{:03}{}", self.min_heat_from[i], CHAR[OPPOSITE[i] as usize]); 
            }
        }
        write!(f, "] ")
    }
}


fn go_direction(x: usize, y: usize, x_max: usize, y_max: usize, direction: u8) -> Option<(usize, usize)> {
    match direction {
        DOWN  => if y < y_max - 1 { return Some((x, y + 1)) },
        UP    => if y >= 1 { return Some((x, y - 1)) },
        RIGHT => if x < x_max - 1 { return Some((x + 1, y)) },
        LEFT  => if x >= 1  { return Some((x - 1, y)) },
        _ => panic!("Invalid direction: '{}'", direction)
    }
    None
}

type Queue = VecDeque<(usize, usize, Direction)>;
fn traverse_graph1(graph: &mut Vec2D::<Node>) -> u32 {
    let mut queue: Queue = VecDeque::from([(0, 0, RIGHT), (0, 0, DOWN)]);

    let xlen= graph.xlen;
    let ylen = graph.ylen;
    
    while !queue.is_empty() {
        let (x, y, direction_from) = queue.pop_front().unwrap();
        let node = *graph.at(x ,y).unwrap();  // copy required because we borrow from graph later :(
        if !node.updated_from[direction_from as usize] { continue; }
        // update surrounding reachable nodes if they can be reached at a lower cost
        // dont check the direction where we came from
        // if updated, queue them
        // println!("At ({},{}) from {}", x, y, CHAR[direction_from as usize]);

        let start_heat_loss = node.min_heat_from[direction_from as usize].min(node.min_heat_from[OPPOSITE[direction_from as usize] as usize]); 
        if direction_from == UP || direction_from == DOWN {
            update_direction(&mut queue, graph, start_heat_loss, x, y, RIGHT, 3, 0);
            update_direction(&mut queue, graph, start_heat_loss, x, y, LEFT, 3, 0);
        }
        else {
            update_direction(&mut queue, graph, start_heat_loss, x, y, UP, 3, 0);
            update_direction(&mut queue, graph, start_heat_loss, x, y, DOWN, 3, 0);
        }
        graph.at_mut(x ,y).unwrap().updated_from[direction_from as usize] = false;
    }
    

    let min_heat_loss = *graph.at(xlen - 1, ylen - 1).unwrap().min_heat_from.iter().min().unwrap();

    return min_heat_loss;
}


fn traverse_graph2(graph: &mut Vec2D::<Node>) -> u32 {
    let mut queue: VecDeque<(usize, usize, Direction)> = VecDeque::from([(0, 0, RIGHT), (0, 0, DOWN)]);
    let xlen= graph.xlen;
    let ylen = graph.ylen;
    
    while !queue.is_empty() {
        let (x, y, direction_from) = queue.pop_front().unwrap();
        let node = *graph.at(x ,y).unwrap();  // copy required because we borrow from graph later :(
        if !node.updated_from[direction_from as usize] { continue; }
        // println!("At ({},{}) from {}", x, y, CHAR[direction_from as usize]);

        let start_heat_loss = node.min_heat_from[direction_from as usize].min(node.min_heat_from[OPPOSITE[direction_from as usize] as usize]); 
        if direction_from == UP || direction_from == DOWN {
            update_direction(&mut queue, graph, start_heat_loss, x, y, RIGHT, 7, 3);
            update_direction(&mut queue, graph, start_heat_loss, x, y, LEFT,  7, 3);
        }
        else {
            update_direction(&mut queue, graph, start_heat_loss, x, y, UP,    7, 3);
            update_direction(&mut queue, graph, start_heat_loss, x, y, DOWN,  7, 3);
        }
        graph.at_mut(x ,y).unwrap().updated_from[direction_from as usize] = false;
    }

    let min_heat_loss = *graph.at(xlen - 1, ylen - 1).unwrap().min_heat_from.iter().min().unwrap();
    return min_heat_loss;
}


fn update_direction(queue: &mut Queue, graph: &mut Vec2D<Node>, mut start_heat_loss: u32, mut start_x: usize, mut start_y: usize, direction: Direction, n_steps: usize, n_skip: usize)  {
    let xlen= graph.xlen;
    let ylen = graph.ylen;
    for _ in 0..n_skip {
        if let Some((other_x, other_y)) = go_direction(start_x, start_y, xlen, ylen, direction) {
            let other_node = graph.at_mut(other_x, other_y).unwrap();
            start_heat_loss += other_node.heat_loss;
            start_x = other_x;
            start_y = other_y;
            // println!("    {} - skipping ({},{}): {} ", CHAR[direction as usize], other_x, other_y, start_heat_loss);
        }
        else {
            // println!("    {} - skipping out of range ", CHAR[direction as usize]);
            return;
        }
    }
    for _ in 0..n_steps {
        if let Some((other_x, other_y)) = go_direction(start_x, start_y, xlen, ylen, direction) {
            // print!("    {} - check ({},{}): {} ", CHAR[direction as usize], other_x, other_y, start_heat_loss);
            let other_node = graph.at_mut(other_x, other_y).unwrap();
            start_heat_loss += other_node.heat_loss;
            if other_node.min_heat_from[direction as usize] > start_heat_loss {  // update all
                other_node.min_heat_from[direction as usize] = start_heat_loss;
                other_node.updated_from[direction as usize] = true;
                queue.push_back((other_x, other_y, direction));
                // println!("+ shorter");
                // dont break because we might reach a shorter node
            }
            // else {
                // println!("X longer");
            // }
            start_x = other_x;
            start_y = other_y;
        }
        else {
            // println!("    {} - out of range ", CHAR[direction as usize]);
            return;
        }
    }
}


fn main() {
    let input = "input.txt";
    // let input = "example.txt";
    let mut lines = read_lines(&input);
    let line_length = lines.next().expect("No line").unwrap().len();
    let n_lines = lines.count() + 1;  // already consumed one
    lines = read_lines(&input);
    let mut city_blocks = Vec2D::<Node>::new(line_length, n_lines, Node::new());
    for (y, line) in lines.map(|r| r.ok().unwrap()).enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            city_blocks.at_mut(x, y).unwrap().heat_loss = (c - b'0') as HeatLoss;
        }
    }
    city_blocks.at_mut(0, 0).unwrap().min_heat_from = [0; 4];
    let min_heat_loss = traverse_graph1(&mut city_blocks.clone());
    // println!("{}", city_blocks);
    println!("Minimum heat loss: (1): {}", min_heat_loss);

    let min_heat_loss = traverse_graph2(&mut city_blocks);
    // println!("{}", city_blocks);
    println!("Minimum heat loss: (2): {}", min_heat_loss);
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

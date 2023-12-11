use std::io::{self, BufRead};

const ADD_TO_EMPTY1: usize = 1;
const ADD_TO_EMPTY2: usize = 1_000_000-1;
fn add_empty_columns(galaxy_positions: &mut Vec<(usize, usize)>, add_to_empty: usize) {
    let mut last_x: usize = galaxy_positions[0].0;
    let mut empty_columns: usize = 0;
    for i in 1..galaxy_positions.len() {
        if galaxy_positions[i].0 > last_x + 1 {
            empty_columns += (galaxy_positions[i].0 - last_x - 1) * add_to_empty;
        }
        last_x = galaxy_positions[i].0;
        galaxy_positions[i].0 += empty_columns;
    }
}

fn get_distance_sum(galaxy_positions: &Vec<(usize, usize)>) -> usize {
    let mut distance_sum: usize = 0;
    for i in 0..galaxy_positions.len() - 1 {
        for j in i+1..galaxy_positions.len() {
            // println!("{}<->{}: {}", i, j, galaxy_positions[i].0.abs_diff(galaxy_positions[j].0) + galaxy_positions[i].1.abs_diff(galaxy_positions[j].1));
            distance_sum += galaxy_positions[i].0.abs_diff(galaxy_positions[j].0);
            distance_sum += galaxy_positions[i].1.abs_diff(galaxy_positions[j].1);
        }
    }
    return distance_sum;
}

fn main() {
    // let input = "example.txt";
    let input = "input.txt";
    let lines = read_lines(&input);
    let mut galaxy_positions1: Vec<(usize, usize)> = Vec::new();
    let mut galaxy_positions2: Vec<(usize, usize)> = Vec::new();
    galaxy_positions1.reserve(300);
    galaxy_positions2.reserve(300);
    let mut y1: usize = 0;
    let mut y2: usize = 0;
    for line in lines.map(|r| r.ok().unwrap()) {
        let mut galaxies = line.chars().enumerate().filter(|(_, c)| *c == '#');
        if let Some((x, _)) = galaxies.next() {
            galaxy_positions1.push((x, y1));
            galaxy_positions2.push((x, y2));
            while let Some((x, _)) = galaxies.next() {
                galaxy_positions1.push((x, y1));
                galaxy_positions2.push((x, y2));
            }
        }
        else {  // empty lines are double the size
            y1 += ADD_TO_EMPTY1;
            y2 += ADD_TO_EMPTY2;
        }
        y1 += 1;
        y2 += 1;
    }
    // sort by x
    galaxy_positions1.sort_unstable_by(|a ,b| a.0.cmp(&b.0));
    galaxy_positions2.sort_unstable_by(|a ,b| a.0.cmp(&b.0));
    add_empty_columns(&mut galaxy_positions1, ADD_TO_EMPTY1);
    add_empty_columns(&mut galaxy_positions2, ADD_TO_EMPTY2);
    let distance_sum1: usize = get_distance_sum(&galaxy_positions1);
    println!("Sum of galaxy distances: (1): {}", distance_sum1);
    let distance_sum2: usize = get_distance_sum(&galaxy_positions2);
    println!("Sum of galaxy distances: (2): {}", distance_sum2);
}


fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

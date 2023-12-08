use std::io::{self, BufRead};
use std::collections::HashMap;

struct Junction(u32, u32);

fn hash(s: &str) -> u32 {
    let mut h: u32 = 0;
    for (i, c) in s.as_bytes().iter().enumerate() {
        h |= (*c as u32) << (8 * i);
        // println!("i: {}, s={}, Char: {:8b}, h: {:32b}", i, s, c, h);
    }
    return h;
}

fn ends_with(h: &u32, c: char) -> bool {
    return (h >> (8 * 2)) == (c as u32);
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    let mut d: u64;
    assert!(x != 0 && y != 0);
    while y != 0 {
        d = y;
        y = x % y;
        x = d;
    }
    return x
}

fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b);
}

fn main() {
    let input = "input.txt";
    // let input = "example.txt";
    let mut lines = read_lines(&input);
    let Ok(instructions) = lines.next().expect("No line found") else { panic!("No line found") };
    lines.next();
    let mut junctions = HashMap::new();
    let mut junctions_a = Vec::new();
    for line in lines.map(|r| r.ok().unwrap()) {
        let junction = hash(&line[0..3]);
        junctions.insert(junction, Junction(hash(&line[7..10]), hash(&line[12..15])));
        if ends_with(&junction, 'A') {
            junctions_a.push(junction);
        }
    }
    let match_junction = |junction: &u32, instruction: &char| -> u32 {
        match instruction {
            'R' => junctions[&junction].1,
            'L' => junctions[&junction].0,
            _ => panic!("Unknown instruction {}", instruction),
        }
    };
    // task 1
    let mut steps: u32 = 0;
    let target = hash("ZZZ");
    let mut junction = hash("AAA");
    println!("Check endswidth {} {} {} {}", ends_with(&hash("AAA"), 'A'), ends_with(&hash("BFZ"), 'Z'), ends_with(&hash("AVD"), 'A'), ends_with(&hash("ZZS"), 'Z'));
    'outer: loop {
        for instruction in instructions.chars() {
            if junction == target { break 'outer; }
            steps += 1;
            junction = match_junction(&junction, &instruction);
        }
    }
    println!("Required steps(1): {}", steps);
    let mut steps_to_z = vec![0 as u32; junctions_a.len()];
    let mut steps_cycle = vec![0 as u32; junctions_a.len()];
    let mut first_z = vec![0 as u32; junctions_a.len()];
    for i in 0..junctions_a.len() {
        steps = 0;
        'outer: loop {
            for instruction in instructions.chars() {
                if ends_with(&junctions_a[i], 'Z') {
                    if steps_to_z[i] == 0 {
                        steps_to_z[i] = steps;
                        first_z[i] = junctions_a[i];
                        steps = 0;
                    }
                    else {
                        assert_eq!(first_z[i], junctions_a[i], "Z->Z is not a cycle!");
                        steps_cycle[i] = steps;
                        break 'outer;
                    }
                }
                // print!("Step {:012}: junction[{}]={:024b}\r", steps, i, junctions_a[i]);
                junctions_a[i] = match_junction(&junctions_a[i], &instruction);
                steps += 1;
            }
        }
    }
    for i in 0..junctions_a.len() {
        println!("A-Z: {}, Z->Z: {}", steps_to_z[i], steps_cycle[i]);
    }
    let mut steps: u64 = steps_cycle[0] as u64;
    for i in 1..steps_cycle.len() {
        print!("lcm({:12}, {:8})", steps, steps_cycle[i]);
        steps = lcm(steps, steps_cycle[i] as u64);
        println!("={}", steps);
    }
    println!("Required steps(2): {}", steps);

}


fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

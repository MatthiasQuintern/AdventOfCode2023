use std::io::{self, BufRead};
use std::fmt::Debug;


fn main() {
    task1();
    task2();
}

// x = (T - t) * t = T * t - t^2
// 0 = t^2 - T * t + x
fn task1() {
    let input = "input.txt";
    let mut lines = read_lines(&input);
    let times = split_line(&lines.next().unwrap().expect("No line found"));
    let distances = split_line(&lines.next().unwrap().expect("No line found"));
    assert_eq!(times.len(), distances.len(), "uneqal array lengths");
    let mut total_ways_mult: i32 = 1;
    for i in 0..times.len() {
        let mut n_ways: i32 = 0;
        for t in 1..times[i] {
            let x: i32 = times[i] * t - t.pow(2);
            // println!("T={}, t={}, x={}", times[i], t, x);
            if x > distances[i] {
                n_ways += 1;
            }
        }
        total_ways_mult *= n_ways;
    }
    println!("Product of number of ways: {}", total_ways_mult);
}

fn task2() {
    let input = "input.txt";
    let mut lines = read_lines(&input);
    let time = parse2(&lines.next().unwrap().expect("No line found"));
    let distance = parse2(&lines.next().unwrap().expect("No line found"));
    // println!("time={}, distance={}", time, distance);
    let t1 = solve_squared(1., -time, distance, 1.);
    let t2 = solve_squared(1., -time, distance, -1.);
    // println!("t1={}, t2={}", t1, t2);
    let lower = t1.min(t2) as u64;
    let upper = t1.max(t2) as u64;
    // println!("lower={}, upper={}", lower, upper);
    println!("Number of possible ways: {}", upper-lower);
    // validation
    // let distance = |t: u64| (time as u64 - t) * t; 
    // println!("distance(t1)={}, distance(t2)={}", distance(lower), distance(upper));
}


fn solve_squared(a: f64, b: f64, c: f64, sign: f64) -> f64 {
    return (-b + sign * (b * b - 4. * a * c).sqrt() as f64) / (2. * a);
}

fn parse2(line: &str) -> f64 {
    let l = rm_char(&line, ' ');
    // println!("{}", l);
    return l[l.find(':').unwrap()+1..].parse::<f64>().unwrap();
}


fn rm_char(original: &str, c: char) -> String {
    return original.chars().filter(|x| x != &c).collect();
}


fn split_line(line: &str) -> Vec<i32> {
    let colon = line.find(':').unwrap_or(0);
    return split_into_numbers::<i32>(&line[colon+2..]).collect();
}

fn split_into_numbers<T: std::str::FromStr>(x: &str) -> impl Iterator<Item = T> + '_ where <T as std::str::FromStr>::Err: Debug {
    return x.split(' ').filter(|&n| {n != "" && n != " "}).map(|n| n.parse::<T>().unwrap());
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

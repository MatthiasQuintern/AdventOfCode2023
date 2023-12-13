use std::io::{self, BufRead};


fn get_diff_bits_count(a: u32, b: u32) -> usize {  // return 0 if equal, 1 if 1 bit is different, else 2
    let diff = a ^ b;
    if diff == 0 { return 0 }
    if (diff & diff.overflowing_sub(1).0) == 0 { return 1 }
    return 2;
}

fn find_reflection_axis(pattern: &Vec<u32>, has_smack: bool) -> Option<usize> {
    for i in 1..pattern.len() {
        let mut found_smack = false;
        let diff = get_diff_bits_count(pattern[i-1], pattern[i]);
        if has_smack && diff == 1 {
            found_smack = true;
        }
        if diff == 0 || found_smack {  // find two identical neighbors (or differing by 1 for task 2
            let mut matching = true;
            for j in 1..i.min(pattern.len() - i) {  // check top and bottom if equal (or diff by 1 bit if has_smack)
                let diff = get_diff_bits_count(pattern[i-1-j], pattern[i+j]);
                if diff == 0 { continue; }
                if has_smack && diff == 1 && !found_smack {  // allow only one smack
                    found_smack = true;
                    continue;
                }
                matching = false; 
                break;
            }
            if matching && (!has_smack || (has_smack && found_smack)) {  // require smack for task 2
                return Some(i);
            }
        }
    }
    return None;
}

fn do_task(horizontal: &Vec<u32>, vertical: &Vec<u32>, has_smack: bool) -> usize {
    let mut mirror_sum: usize = 0;
    if let Some(x) = find_reflection_axis(&horizontal, has_smack) {
        mirror_sum += x * 100;
    }
    if let Some(x) = find_reflection_axis(&vertical, has_smack) {
        mirror_sum += x;
    }
    return mirror_sum;
}



fn main() {
    // let input = "example.txt";
    let input = "input.txt";
    let lines = read_lines(&input);
    let mut horizontal: Vec<u32> = Vec::new();
    let mut vertical: Vec<u32> = Vec::new();
    horizontal.reserve(20);
    vertical.reserve(20);
    let mut mirror_sum1: usize = 0;
    let mut mirror_sum2: usize = 0;
    let convert = |c| -> u32 { if c == '#' { 1 } else { 0 } };
    for line in lines.map(|r| r.ok().unwrap()) {
        if line.len() == 0 {
            if horizontal.len() != 0 {  // skip first time
                mirror_sum1 += do_task(&horizontal, &vertical, false);
                mirror_sum2 += do_task(&horizontal, &vertical, true);
            }
            horizontal.clear();
            vertical.clear();
            continue;
        }
        if horizontal.len() == 0 {  // 0 for each column
            (0..line.len()).for_each(|_| vertical.push(0));
        }
        horizontal.push(0);
        for (i, n) in line.chars().map(|c| convert(c)).enumerate() {
            *horizontal.last_mut().unwrap() |= (n << i);
            vertical[i] |= (n << (horizontal.len()-1));
        }
    }
    mirror_sum1 += do_task(&horizontal, &vertical, false);
    mirror_sum2 += do_task(&horizontal, &vertical, true);
    println!("Sum: (1): {}", mirror_sum1);
    println!("Sum: (2): {}", mirror_sum2);
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}


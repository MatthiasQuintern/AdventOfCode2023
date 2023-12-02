use std::io::{self, BufRead};

fn main() {
    let max_red:    u32 = 12;
    let max_green:  u32 = 13;
    let max_blue:   u32 = 14;

    let lines = read_lines("input.txt");
    let mut sum_ind_possible: u32 = 0;
    let mut sum_ind_minpower: u32 = 0;
    for line in lines {
        let mut game_possible = true;
        let Ok(game) = line else { panic!("Line not ok"); };
        let Some(colon) = game.find(':') else { panic!("Could not find ':'"); };
        let game_index = &game[5..colon].parse::<u32>().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for samples in game[colon+2..].split("; ") {
            for color_value in samples.split(", ") {
                let mut iter = color_value.split(' ');
                let count; 
                if let Some(c) = iter.next() { count = c.parse::<u32>().unwrap(); } else { panic!("Invalid line: {}", game); };
                let Some(color) = iter.next() else { panic!("Invalid line: {}", game); };
                // println!("Color={}, value={}", color, count);
                let do_color = |value: u32, colorvalue: &mut u32, max_color: u32| {
                    if value > *colorvalue {
                        *colorvalue = value;
                    }
                    return value <= max_color;
                };
                game_possible &= match color {
                    "red"   => do_color(count, &mut red, max_red),
                    "green" => do_color(count, &mut green, max_green),
                    "blue"  => do_color(count, &mut blue, max_blue),
                    _ => false,
                };
            }
        }
        if game_possible { sum_ind_possible += game_index };
        sum_ind_minpower += red * green * blue;
    }
    println!("Sum of indices of possible games: {}", sum_ind_possible);
    println!("Sum of power of minimum sets: {}", sum_ind_minpower);
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
// where P: AsRef<std::path::Path>, {
//     let file = std::fs::File::open(filename)?;
//     Ok(std::io::BufReader::new(file).lines())
// }

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}


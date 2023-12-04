use std::io::{self, BufRead};

fn split_into_numbers(x: &str) -> impl Iterator<Item = i32> + '_ {
    return x.split(' ').filter(|&n| {n != "" && n != " "}).map(|n| n.parse::<i32>().unwrap());
}

fn main() {
    let input = "input.txt";
    let lines = read_lines(&input);
    let n_cards: usize = read_lines(&input).count();
    let mut cards: Vec<u32> = vec![1; n_cards];
    let mut sum_points: u32 = 0;
    // number of copies per card
    for (i, line) in lines.enumerate() {
        let mut points: u32 = 0;
        let mut n_matches: usize = 0;
        let Ok(card) = line else { panic!("Line not ok"); };
        let Some(colon) = card.find(':') else { panic!("Could not find ':'"); };
        let Some(pipe) = card.find('|') else { panic!("Could not find '|'"); };
        // let split_into_numbers = |x: &str| { return x.split(' ').filter(|&n| {n != "" && n != " "}).map(|n| n.parse::<i32>().unwrap()); };
        let winning_numbers: Vec<i32> = split_into_numbers(&card[colon+2..pipe-1]).collect();
        for number in split_into_numbers(&card[pipe+2..]) {
            if winning_numbers.contains(&number) {
                // task 1
                if points == 0 { points = 1; }
                else { points <<= 1; }
                // task 2
                if points == 0 { points = 1; }
                n_matches += 1;
            }
        }
        // task 1
        sum_points += points;
        // task 2: add #copies(card i) to the next n_matches cards
        for j in i+1..i+1+n_matches {
            cards[j] += cards[i];
        }
    }
    println!("Total points: {}", sum_points);
    println!("Total number of cards: {}", cards.iter().sum::<u32>());
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

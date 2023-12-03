use std::io::BufRead;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Symbol(char, usize, usize);


fn main() {
    let schematic = read_lines("input.txt");
    let mut part_sum = 0;
    let mut gear_ratio_sum: u32 = 0;
    let mut first_gears = HashMap::new();  // Symbol: first_gear
    for (n_line, line) in schematic.iter().enumerate() {
        // println!("{} {}", n_line, line);
        let mut num_begin: usize = usize::MAX;
        let mut num_end: usize = usize::MAX;
        for (n_char, c) in line.char_indices() {
            if num_begin == usize::MAX {
                if is_digit(c) { num_begin = n_char; }
            } 
            else {  // in number
                if !is_digit(c) {
                    num_end = n_char - 1;
                }
                else if n_char == line.chars().count() - 1 {  // last char
                    num_end = n_char;
                }
                if num_end != usize::MAX {
                    let symbol = find_symbol(&schematic, n_line, num_begin, n_char-1);
                    if symbol != NO_SYMBOL {
                        let num = line[num_begin..=num_end].parse::<u32>().expect("Could not parse number");  
                        part_sum += num;
                        if symbol.0 == '*' {
                            if first_gears.contains_key(&symbol) {
                                gear_ratio_sum += first_gears[&symbol] * num;
                            }
                            else { first_gears.insert(symbol, num); }
                        }
                    }
                    num_begin = usize::MAX;
                    num_end = usize::MAX;
                }
            }
        }
    }
    println!("Total sum of part numbers: {}", part_sum);
    println!("Total sum of gear ratios: {}", gear_ratio_sum);

}


fn is_digit(c: char) -> bool {
    return '0' <= c && c <= '9'
}
fn is_symbol(c: char) -> bool {
    return c != '.' && (!is_digit(c));
}

const NO_SYMBOL:Symbol = Symbol('\0', usize::MAX, usize::MAX);
// return symbol and its position
fn find_symbol(schematic: &Vec<String>, num_line: usize, num_begin: usize, num_end: usize) -> Symbol {
    // search char left and right
    if num_begin > 0 {
        let c = schematic[num_line].chars().nth(num_begin-1).unwrap();
        if is_symbol(c) { return Symbol(c, num_line, num_begin - 1); }
    }
    if num_end + 1 < schematic[num_line].chars().count() {
        let c = schematic[num_line].chars().nth(num_end+1).unwrap();
        if is_symbol(c) { return Symbol(c, num_line, num_end + 1); }
    }

    let skip: usize = num_begin.saturating_sub(1);
    let take: usize;
    if num_begin == 0 {
        take = num_end.saturating_sub(num_begin) + 2;
    } else {
        take = num_end.saturating_sub(num_begin) + 3;
    }

    // search top line
    if num_line > 0 {
        for (i, c) in schematic[num_line-1].char_indices().skip(skip).take(take) {
            if is_symbol(c) { return Symbol(c, num_line-1, i); }
        }
    }
    // search bottom line
    if num_line + 1 < schematic.len() {
        for (i, c) in schematic[num_line+1].char_indices().skip(skip).take(take) {
            if is_symbol(c) { return Symbol(c, num_line+1, i); }
        }
    }
    return NO_SYMBOL;
}


fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines().map(|l| {l.expect("Could not parse line")}).collect(),
    };
}

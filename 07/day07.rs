use std::fmt;
use std::io::{self, BufRead};
use std::cmp::{Ord, Ordering};

const HIGH_CARD: u32     = 0;
const ONE_PAIR: u32      = 1;
const TWO_PAIR: u32      = 2;
const THREE_OF_KIND: u32 = 3;
const FULL_HOUSE: u32    = 4;
const FOUR_OF_KIND: u32  = 5;
const FIVE_OF_KIND: u32  = 6;


#[derive(Eq)]
struct Hand {
    cards: Vec<usize>,
    hand_type: u32,
    bid: u32,
}
impl Hand {
    fn new1(cards: &str, bid: u32, card_counts: &mut [usize; 13]) -> Hand {
        let mut hand = Hand{ cards: cards.chars().map(|c| Hand::char_to_num1(c)).collect(), hand_type: HIGH_CARD, bid: bid };
        assert!(hand.cards.len() == 5, "Found {} cards instead of five", hand.cards.len());
        hand.determine_type1(card_counts);
        return hand;
    }
    fn new2(cards: &str, bid: u32, card_counts: &mut [usize; 13]) -> Hand {
        let mut hand = Hand{ cards: cards.chars().map(|c| Hand::char_to_num2(c)).collect(), hand_type: HIGH_CARD, bid: bid };
        assert!(hand.cards.len() == 5, "Found {} cards instead of five", hand.cards.len());
        hand.determine_type2(card_counts);
        return hand;
    }
    fn char_to_num1(c: char) -> usize {
        match c {
            '2' => return 0,
            '3' => return 1,
            '4' => return 2,
            '5' => return 3,
            '6' => return 4,
            '7' => return 5,
            '8' => return 6,
            '9' => return 7,
            'T' => return 8,
            'J' => return 9,
            'Q' => return 10,
            'K' => return 11,
            'A' => return 12,
            _ => panic!("Invalid char found: {}", c),
        }
    }
    fn char_to_num2(c: char) -> usize {
        match c {
            'J' => return 0,
            '2' => return 1,
            '3' => return 2,
            '4' => return 3,
            '5' => return 4,
            '6' => return 5,
            '7' => return 6,
            '8' => return 7,
            '9' => return 8,
            'T' => return 9,
            'Q' => return 10,
            'K' => return 11,
            'A' => return 12,
            _ => panic!("Invalid char found: {}", c),
        }
    }
    fn determine_type1(&mut self, card_counts: &mut [usize; 13]) {
        // use given array to reduce memory allocations
        for count in card_counts.iter_mut() { *count = 0 };
        for card in &self.cards {
            card_counts[*card] += 1;
        }
        let mut found_three = false;
        let mut found_two = false;
        for count in card_counts {
            match count {
                5 => { self.hand_type = FIVE_OF_KIND; return; },
                4 => { self.hand_type = FOUR_OF_KIND; return; },
                3 => { self.hand_type = THREE_OF_KIND; found_three = true; },
                2 => { 
                    if found_two {
                        self.hand_type = TWO_PAIR;
                        return;
                    }
                    else {
                        self.hand_type = ONE_PAIR; 
                        found_two = true; 
                    }
                },
                _ => {},
            }
            if found_two && found_three {
                self.hand_type = FULL_HOUSE;
            }
        }
    }
    fn determine_type2(&mut self, card_counts: &mut [usize; 13]) {
        // better solution would be to just add the number of jokers to the card with the highest
        // number 
        // use given array to reduce memory allocations
        for count in card_counts.iter_mut() { *count = 0 };
        for card in &self.cards {
            card_counts[*card] += 1;
        }
        let mut found_three = false;
        let mut found_two = false;
        // determine without jokers
        for count in card_counts.iter().skip(1) {
            match count {
                5 => { self.hand_type = FIVE_OF_KIND; return; },
                4 => { self.hand_type = FOUR_OF_KIND; break; },
                3 => { self.hand_type = THREE_OF_KIND; found_three = true; },
                2 => { 
                    if found_two {
                        self.hand_type = TWO_PAIR;
                        break;
                    }
                    else {
                        self.hand_type = ONE_PAIR; 
                        found_two = true; 
                    }
                },
                _ => {},
            }
            if found_two && found_three {
                self.hand_type = FULL_HOUSE;
                return
            }
        }
        match card_counts[0] {
            0 => return, // no jokers
            1 => match self.hand_type {
                    FOUR_OF_KIND => self.hand_type = FIVE_OF_KIND,
                    THREE_OF_KIND => self.hand_type = FOUR_OF_KIND,
                    TWO_PAIR => self.hand_type = FULL_HOUSE,
                    ONE_PAIR => self.hand_type = THREE_OF_KIND,
                    HIGH_CARD => self.hand_type = ONE_PAIR,
                    _ => panic!("Unhandled joker(1) case: hand_type={}", self.hand_type),
                },
            2 => match self.hand_type {
                    THREE_OF_KIND => self.hand_type = FIVE_OF_KIND,
                    ONE_PAIR => self.hand_type = FOUR_OF_KIND,
                    HIGH_CARD => self.hand_type = THREE_OF_KIND,
                    _ => panic!("Unhandled joker(2) case: hand_type={}", self.hand_type)
                },
            3 => match self.hand_type {
                    ONE_PAIR => self.hand_type = FIVE_OF_KIND,
                    HIGH_CARD => self.hand_type = FOUR_OF_KIND,
                    _ => panic!("Unhandled joker(3) case: hand_type={}", self.hand_type)
                },
            _ => self.hand_type = FIVE_OF_KIND
        }
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut order = self.hand_type.cmp(&other.hand_type);
        if order != Ordering::Equal { return order };
        for i in 0..self.cards.len() {
            order = self.cards[i].cmp(&other.cards[i]);
            if order != Ordering::Equal { return order };
        }
        panic!("Equal elements: {} and {}", self, other);
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ =write!(f, "Hand=");
        for c in &self.cards {
            let _ = write!(f, "{}", c);
        }
        write!(f, ", type={}, bid={}", self.hand_type, self.bid)
    }
}


fn main() {
    let input = "input.txt";
    let lines = read_lines(&input);
    let mut hands1: Vec<Hand> = Vec::new();
    hands1.reserve(1000);
    let mut hands2: Vec<Hand> = Vec::new();
    hands2.reserve(1000);
    // use given array to reduce memory allocations
    let mut card_counts: [usize; 13] = [0; 13];
    for (_, line) in lines.enumerate() {
        let Ok(line) = line else { panic!("Line not ok"); };
        hands1.push(Hand::new1(&line[..5], line[6..].parse::<u32>().unwrap(), &mut card_counts));
        hands2.push(Hand::new2(&line[..5], line[6..].parse::<u32>().unwrap(), &mut card_counts));
        // println!("Hand: {}", hands.last().expect("No hand"));
    }
    hands1.sort_unstable();
    hands2.sort_unstable();
    let mut total_winnings1: u32 = 0;
    let mut total_winnings2: u32 = 0;
    for (i, hand) in hands1.iter().enumerate()  {
        total_winnings1 += (i as u32 + 1) * hand.bid;
    }
    for (i, hand) in hands2.iter().enumerate()  {
        total_winnings2 += (i as u32 + 1) * hand.bid;
    }
    println!("Total winnings (1): {}", total_winnings1);
    println!("Total winnings (2): {}", total_winnings2);
}


fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

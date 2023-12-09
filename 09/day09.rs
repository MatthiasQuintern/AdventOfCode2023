use std::io::{self, BufRead};
use std::fmt::Debug;
use std::fmt;

// 0 1 3 6 10 ...
fn triangular_numbers(n: usize) -> usize { (n * (n + 1)) / 2 }

struct Triangle {
    data: Vec::<i32>,
    width: usize,
}
impl Triangle {
    fn new(firstrow: &mut Vec<i32>) -> Triangle {
        let mut triangle = Triangle{data: Vec::new(), width: firstrow.len()};
        triangle.data = std::mem::take(firstrow);
        triangle.data.resize(triangular_numbers(triangle.width), 0);
        return triangle
    }
    fn at_mut(&mut self, row: usize, col: usize) -> &mut i32 {
        let i = self._index(row, col);
        &mut self.data[i]
    }
    fn at(&self, row: usize, col: usize) -> i32 {
        self.data[self._index(row, col)]
    }
// 1 2 3 4
// 1 2 3
// 1 2
// 1
    fn _index(&self, row: usize, col: usize) -> usize {
        assert!(col < self.row_len(row), "Invalid indices for Triangle with width {}: row={}, col={}", self.width, row, col);
        return col + self.data.len() - triangular_numbers(self.width - row);  // subtract  the triangle with width (self.width - row) from total length
    }
    // fn row(&mut self, row: usize) -> impl Iterator<Item = &mut i32> {
    //     assert!(row < self.width, "Invalid row");
    //     let skip: usize = self.width * row - triangular_numbers(self.width - row);  // skip <row> rows - the triangle with width (self.width - row)
    //     self.data.iter_mut().skip(skip).take(self.row_len(row));
    // }
    // #rows or #colums of first row
    fn len(&self) -> usize { self.width }
    fn row_len(&self, row: usize) -> usize { self.width - row }
}
impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.len() {
            for c in 0..self.row_len(r) {
                let res = write!(f, "{} ", self.at(r, c));
                if res.is_err() { return res };
            }
            if r != self.len() - 1 { println!() }
        }
        return Result::Ok(());
    }
}

fn predict(numbers: &mut Vec<i32>) -> (i32, i32) {
    numbers.insert(0, 0);
    numbers.push(0); 
    let mut triangle = Triangle::new(numbers);
    // fill until maxrow is all zero
    let mut maxrow: usize = triangle.len() - 1;  // skip last row
    for r in 1..maxrow {
        maxrow = r;
        let mut allzero = true;
        for j in 1..triangle.row_len(r) - 1 {
            // print!("{} -> ", triangle.at(r, j));
            let diff = triangle.at(r-1, j+1) - triangle.at(r-1, j); //).abs();
            allzero &= diff == 0;
            *triangle.at_mut(r, j) = diff;
        }
        if allzero { break }
        // println!();
    }
    assert!(maxrow < triangle.len()- 3, "maxrow={} ", maxrow);
    for r in (0..maxrow).rev() {
        // forward prediction
        let newval = triangle.at(r, triangle.row_len(r) - 2) + triangle.at(r + 1, triangle.row_len(r + 1) - 1);
        *triangle.at_mut(r, triangle.row_len(r) - 1) = newval;

        let newval = triangle.at(r, 1) - triangle.at(r + 1, 0);
        *triangle.at_mut(r, 0) = newval;
    }
    // println!("{}", triangle);
    return (triangle.at(0, 0), triangle.at(0, triangle.row_len(0) - 1));
}

fn main() {
    let input = "input.txt";
    // let input = "example.txt";
    let lines = read_lines(&input);
    let mut sum_forward_predictions = 0;
    let mut sum_backward_predictions = 0;
    for line in lines.map(|r| r.ok().unwrap()) {
        let preds = predict(&mut split_into_numbers::<i32>(&line).collect());
        sum_backward_predictions += preds.0;
        sum_forward_predictions += preds.1;
    }
    println!("OASIS: sum of forward  predictions (1): {}", sum_forward_predictions);
    println!("OASIS: sum of backward predictions (2): {}", sum_backward_predictions);
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

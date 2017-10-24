/*Rust implementation for the Stone Game Problem*/

enum Convert {
    First = 1, 
    Second = -1,
}

fn p1(c: i32, moves: Vec<i32>) -> i32 {
    let mut max_result = Convert::Second as i32;
    let mut result: i32;
    for m in &moves {
        if c - *m >= 0 {
            result = p2(c-m, moves.to_vec());
            if result > max_result { max_result = result; }
        }
    }
    max_result
}

fn p2(c: i32, moves: Vec<i32>) -> i32 {
    let mut min_result = Convert::First as i32;
    let mut result: i32;
    for m in &moves {
        if c - *m >= 0 {
            result = p1(c-m, moves.to_vec());
            if result < min_result { min_result = result; }
        }
    }
    min_result
}

fn main() {
    let cases = [1,2,3,4,5,6,7,10].to_vec();
    let moves = [2,3,5].to_vec();
    let mut outcome: i32;
    
    for c in cases {
         outcome = p1(c, moves.to_vec());
         match outcome {
            1 => println!("{}:\tFirst", c),
            -1 => println!("{}:\tSecond", c),
            _ => panic!("An internal error occurred")
         }
    }
    
}

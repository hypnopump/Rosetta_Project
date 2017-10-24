/*
 * The Problem
 * -----------
 * Two players (numbered 1 and 2) are playing a game with n stones. 
 * Player 1 always plays first, and the two players move in alternating turns. 
 * The game's rules are as follows:
 * (*) In a single move, a player can remove either 2, 3, or 5 stones from the game board.
 * (*) If a player is unable to make a move, that player loses the game.
 * Given the number of stones, find and print the name of the winner (i.e., or ) on a new line.
 * Each player plays optimally, meaning they will not make a move that causes them to lose the game if some better, 
 * winning move exists.
 *
 * Aditional Info:
 * If n = 10 , P1 can remove either 2 or 3 stones to win the game, so we print First on a new line. 
 * Recall that each player moves optimally, so P1 will not remove 5 stones because doing so would cause P1 to lose the game.
 *
 * Desired Output / Test Cases
 * ---------------------------
 * **Input:** List containing test cases: [1,2,3,4,5,6,7,10]
 * **Output:**
 * 1:  Second
 * 2:  First
 * 3:  First
 * 4:  First
 * 5:  First
 * 6:  First
 * 7:  Second
 * 10: First
 */

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

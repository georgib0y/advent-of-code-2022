use core::panic;

const WIN: usize = 6;
const LOSS: usize = 0;
const DRAW: usize = 3;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3; 

pub fn day2() {
    let input = std::fs::read_to_string("day2").unwrap();

    let sum = input.lines()
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .fold(0, |sum, (opp, wld)| sum + score(opp, wld));

    println!("Sum = {sum}");
}

fn score(opp: char, wld: char) -> usize {
    match opp {
        'A' => { // rock
            match wld { 
                'X' => LOSS + SCISSORS,
                'Y' => DRAW + ROCK,
                'Z' => WIN + PAPER,
                _ => panic!("invalid wld char")
            }
        }
        'B' => { // paper
            match wld {
                'X' => LOSS + ROCK,
                'Y' => DRAW + PAPER,
                'Z' => WIN + SCISSORS,
                _ => panic!("invalid wld char")
            }
        }
        'C' => { // scissors
            match wld {
                'X' => LOSS + PAPER,
                'Y' => DRAW + SCISSORS,
                'Z' => WIN + ROCK,
                _ => panic!("invalid wld char")
            }
        }
        _ => panic!("invalid opponent char")
    }
}
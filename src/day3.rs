use std::collections::HashSet;

pub fn day3() {
    let input = std::fs::read_to_string("day3").unwrap();

    let sum = input.lines().step_by(3)
        .zip(input.lines().skip(1).step_by(3))
        .zip(input.lines().skip(2).step_by(3))
        .fold(0, |sum, ((r1, r2), r3)| sum + get_badge_priority(r1, r2, r3));


    // let sum = input.lines().fold(0, |sum, line| sum + get_dup_priority(line));

    println!("{sum}");
}

fn get_badge_priority(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> usize {
    let r1_chars: HashSet<char> = HashSet::from_iter(rucksack1.chars());
    let r2_chars: HashSet<char> = HashSet::from_iter(rucksack2.chars().filter(|ch| r1_chars.contains(&ch)));
    let r3_chars: HashSet<char> = HashSet::from_iter(rucksack3.chars().filter(|ch| r2_chars.contains(&ch)));

    ch_priority(r3_chars.into_iter().next().unwrap())
}

fn ch_priority(ch: char) -> usize {
    if ch.is_lowercase() {
        ch as usize - 'a' as usize + 1
    } else if ch.is_uppercase() {
        ch as usize - 'A' as usize + 27
    } else {
        panic!("char is not lowercase or uppercase")
    }
}
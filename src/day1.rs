#![allow(unused)]
pub fn day1() {
    let input = std::fs::read_to_string("day1").unwrap();
    let mut lines = input.lines();

    let mut max_cals = [0;3];

    let mut cals = 0;


    while let Some(line) = lines.next() {
        if line.trim().is_empty() { 
            shuffle_down(&mut max_cals, cals);
            cals = 0;
            continue;
        }

        cals += line.parse::<i32>().unwrap();
    }

    println!("Top 3 max cals sum = {}", max_cals.into_iter().reduce(|sum, cal| sum+cal).unwrap());
}

fn shuffle_down(max_cals: &mut [i32], mut cals: i32) {
    for max in max_cals {
        if *max < cals {
            let t = *max;
            *max = cals;
            cals = t;
        }
    }
}
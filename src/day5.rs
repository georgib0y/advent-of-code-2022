//     [S] [C]         [Z]            
// [F] [J] [P]         [T]     [N]    
// [G] [H] [G] [Q]     [G]     [D]    
// [V] [V] [D] [G] [F] [D]     [V]    
// [R] [B] [F] [N] [N] [Q] [L] [S]    
// [J] [M] [M] [P] [H] [V] [B] [B] [D]
// [L] [P] [H] [D] [L] [F] [D] [J] [L]
// [D] [T] [V] [M] [J] [N] [F] [M] [G]
//  1   2   3   4   5   6   7   8   9 
pub fn day5() {
    let mut stacks = vec![
        String::new(),
        String::from("DLJRVGF"),
        String::from("TPMBVHJS"),
        String::from("VHMFDGPC"),
        String::from("MDPNGQ"),
        String::from("JLHNF"),
        String::from("NFVQDGTZ"),
        String::from("FDBL"),
        String::from("MJBSVDN"),
        String::from("GLD"),
    ];

    let input = std::fs::read_to_string("day5").unwrap();

    for (line_num, (amount, from, to)) in input.lines().map(process_move).enumerate() {
        println!("line {} amount {amount} from {from} to {to}", line_num+1);
        let mut moved = String::new();
        for _ in 0..amount { moved = format!("{}{moved}", stacks[from].pop().unwrap()); }
        stacks[to].push_str(&moved);
    }

    let mut output = String::new();
    stacks.iter_mut().for_each(|stack| if let Some(c) = stack.pop() {
        output.push(c)
    });

    println!("{output}")
}

fn print_stacks(stacks: &Vec<String>) {
    stacks.iter().enumerate().skip(1)
        .for_each(|(i, stack)| println!("{i}: {stack}"));
    println!();
}

fn process_move(line: &str) -> (usize, usize, usize) {
    let mut split = line.split(' ');
    let amount  = split.nth(1).unwrap().parse::<usize>().unwrap();
    let from    = split.nth(1).unwrap().parse::<usize>().unwrap();
    let to      = split.nth(1).unwrap().parse::<usize>().unwrap();

    (amount, from, to)
}
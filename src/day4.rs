pub fn day4() {
    let input = std::fs::read_to_string("day4").unwrap();

    let count = input.lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(range1, range2)| (range_from_str(range1), range_from_str(range2)))
        .filter(|(r1, r2)| overlaps(*r1, *r2))
        .count();

    println!("{count}")
}

fn range_from_str(range: &str) -> (usize, usize) {
    let (start, end) = range.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

fn overlaps(r1: (usize, usize), r2: (usize, usize)) -> bool {
    if r1.0 == r2.0 { return true }
    let (lowest, highest) = if r1.0 < r2.0 { (r1, r2) } else { (r2, r1) };
    dbg!(lowest, highest);
    lowest.1 >= highest.0
}

fn _fully_contains(r1: (usize, usize), r2: (usize, usize)) -> bool {
    if r1.0 == r2.0 { return true }
    let (lowest, highest) = if r1.0 < r2.0 { (r1, r2) } else { (r2, r1) };
    dbg!(lowest, highest);
    lowest.1 >= highest.1
}
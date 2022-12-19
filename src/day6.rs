use std::collections::{VecDeque, HashSet};

pub fn day6() {
    let input = std::fs::read_to_string("day6").unwrap();

    let mut marker: VecDeque<char> = input.chars().take(14).collect();
    let mut marker_idx = 0;

    for (idx, ch) in input.char_indices().skip(14) {
        if is_unique(&marker) {
            marker_idx = idx;
            break;
        }

        marker.pop_front();
        marker.push_back(ch)
    }

    println!("Marker index = {marker_idx}");
}

fn is_unique(marker: &VecDeque<char>) -> bool {
    let mut chars = HashSet::with_capacity(4);
    for ch in marker {
        if !chars.insert(ch) { return false }
    }

    true
}
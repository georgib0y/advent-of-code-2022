pub fn day8() {
    let input = std::fs::read_to_string("day8").unwrap();
    // let input = String::from("30373\n25512\n65332\n33549\n35390");
    let forest: Vec<Vec<u8>> = input.lines()
        .map(|row| row.chars().map(|ch| ch as u8 - '0' as u8).collect())
        .collect();
    
    // dbg!(&forest);
    // dbg!(is_visible(&forest, 2, 1));
    // dbg!(scenic_score(&forest, 0, 0));
    // dbg!(scenic_score(&forest, 0, 4));
    // dbg!(scenic_score(&forest, 4, 0));
    // dbg!(scenic_score(&forest, 4, 4));
    // dbg!(scenic_score(&forest, 1, 2));
    // dbg!(scenic_score(&forest, 3, 1));
    // let total_visible = (0..forest.len()).fold(0, |total, row| 
    //         total + (0..forest[row].len()).filter(|col| is_visible(&forest, row, *col)).count()
    // );        

    // println!("Total visible: {total_visible}");

    let mut best_score = 0;
    for row in 0..forest.len() {
        for col in 0..forest[0].len() {
            let score = scenic_score(&forest, row, col);
            if score > best_score { best_score = score }
        }
    }

    println!("bestscore = {best_score}");
}

fn scenic_score(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let tree_height = forest[row][col];

    let mut up_score = 1;
    for up in (1..row).rev() {
        if forest[up][col] >= tree_height { break; } 
        up_score += 1;
    }

    let mut down_score = 1;
    for down in row+1..forest.len()-1 {
        if forest[down][col] >= tree_height { break; } 
        down_score += 1;
    }

    let mut left_score = 1;
    for left in (1..col).rev() {
        if forest[row][left] >= tree_height { break; } 
        left_score += 1;
    }

    let mut right_score = 1;
    for right in col+1..forest[0].len()-1 {
        if forest[row][right] >= tree_height { break; } 
        right_score += 1;
    }

    dbg!(up_score, down_score, left_score, right_score);

    up_score * down_score * left_score * right_score
}





fn _is_visible(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    if row == 0 || row == forest.len()-1 { return true }
    if col == 0 || col == forest[0].len()-1 { return true }
    
    let tree_height = forest[row][col];
    // dbg!(row, col, tree_height);
    let mut visible = true;

    // test up
    for up in 0..col {
        if forest[row][up] >= tree_height {
            visible = false;
            break;
        }
    }

    if visible { return true } else { visible = true }

    // test down
    for down in col+1..forest[0].len() {
        if forest[row][down] >= tree_height {
            visible = false;
            break;
        }
    }

    if visible { return true } else { visible = true }

    // test left 
    for left in 0..row {
        if forest[left][col] >= tree_height {
            visible = false;
            break;
        }
    }

    if visible { return true } else { visible = true }

    // test right
    for right in row+1..forest.len() {
        if forest[right][col] >= tree_height {
            visible = false;
            break;
        }
    }

    visible
}

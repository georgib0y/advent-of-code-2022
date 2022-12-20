use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Dir {
    dir_size: usize,
    inclusive_size: usize,
    subdirs: Vec<String>,
    parent_dir: String 
}

impl Dir {
    fn new(parent_dir: &str) -> Dir {
        Dir {
            dir_size: 0, 
            inclusive_size: 0,
            subdirs: Vec::new(), 
            parent_dir: parent_dir.to_string() 
        }
    }

    fn add_subdir(&mut self, subdir: &str) { self.subdirs.push(subdir.to_string(), ) }

    fn add_file(&mut self, file_size: usize) { self.dir_size += file_size }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let subdirs = self.subdirs.iter()
            .fold(String::new(), |s, subdirs| format!("{subdirs} {s}"));

        write!(f, "dir size: {:<10}\tinc. size: {:<10}\tsubdirs: {}\tparent dir: {}", self.dir_size, self.inclusive_size, subdirs, self.parent_dir)
    }
}

pub fn day7() {
    let input = std::fs::read_to_string("day7").unwrap();
    // let input = String::from("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k");
    let mut dirs: HashMap<String, Dir> = HashMap::new();

    // let mut curr_dir = String::from("/");
    let mut curr_dir = String::new();
    // dirs.insert(curr_dir.clone(), Dir::new(&curr_dir));

    for cmd in input.split('$').map(|cmd| cmd.trim()) {
        dbg!(cmd);
        dbg!(&curr_dir);
        // print_dirs(&dirs);       
        if cmd.trim().starts_with("cd") {
            process_cd(cmd, &mut dirs, &mut curr_dir);
        } else if cmd.trim().starts_with("ls"){
            process_ls(cmd, &mut dirs, &mut curr_dir);
        }
    }

    print_dirs(&dirs);       

    calc_inclusive_sizes(&String::from("/"), &mut dirs);

    print_dirs(&dirs);       

    // let sum = dirs.iter().map(|(_, dir)| dir.inclusive_size)
    //     .filter(|inclusive_size| *inclusive_size <= 100000)
    //     .fold(0, |sum, inclusive_size| sum + inclusive_size);

    // println!("sum is: {sum}");

    let total_used = dirs.get("/").unwrap().inclusive_size;

    let smallest = dirs.iter().map(|(_, dir)| dir.inclusive_size)
        .filter(|inc_size| 70000000 - total_used + inc_size >= 30000000 )
        .reduce(|smallest, inc_size| if smallest < inc_size { smallest } else { inc_size })
        .unwrap();

    println!("{smallest}");
}

fn process_cd(cmd: &str, dirs: &mut HashMap<String, Dir>, curr_dir: &mut String) {
    let (_, cd_dir) = cmd.split_once(' ').unwrap();

    if cd_dir.trim() == ".." {
        *curr_dir = dirs.get(curr_dir).unwrap().parent_dir.to_string();
    } else if cd_dir.trim() == "/" {
        *curr_dir = String::from("/");
        dirs.insert(curr_dir.clone(), Dir::new(&curr_dir));
    } else if curr_dir == "/" {
        *curr_dir = format!("/{cd_dir}");
        dirs.insert(curr_dir.clone(), Dir::new("/"));
    } else {
        let new_dir = format!("{curr_dir}/{}", cd_dir.trim().to_string());

        dirs.insert(new_dir.clone(), Dir::new(&curr_dir));
        *curr_dir = new_dir;
    }
}

fn process_ls(cmd: &str, dirs: &mut HashMap<String, Dir>, curr_dir: &mut String) {
    for line in cmd.lines().skip(1) {
        let (file_size, subdir) = line.split_once(' ').unwrap();
        dbg!(&curr_dir);
        if line.starts_with("dir") {
            if curr_dir == "/" {
                dirs.get_mut(curr_dir).unwrap().add_subdir(&format!("/{subdir}"));
            } else {
                dirs.get_mut(curr_dir).unwrap().add_subdir(&format!("{curr_dir}/{subdir}"));
            }
            // dirs.insert(format!("{curr_dir}/{subdir}"), Dir::new(&curr_dir));
            continue;
        }

        dirs.get_mut(curr_dir).unwrap().add_file(file_size.parse().unwrap());
    }
}

fn calc_inclusive_sizes(curr_dir: &String, dirs: &mut HashMap<String, Dir>) -> usize {
    dbg!(curr_dir);
    let mut size = dirs.get(curr_dir).unwrap().dir_size;

    for subdir in dirs.get(curr_dir).unwrap().subdirs.clone() {
        size += calc_inclusive_sizes(&subdir, dirs);
    }

    dirs.get_mut(curr_dir).unwrap().inclusive_size = size;
    size
}

fn print_dirs(dirs: &HashMap<String, Dir>) {
    dirs.iter().for_each(|(hash, dir)| println!("{hash}:\t{dir}"));
    println!();
}
#[derive(Debug)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct Move {
    dir: Dir,
    count: usize,
}

fn main() {
    let raw_data = std::fs::read_to_string("inputs/01").unwrap();
    println!("{}", raw_data);
    let moves: Vec<Move> = raw_data
        .lines()
        .map(|line| {
            let dir_raw = line.chars().next().unwrap();
            let dir = match dir_raw {
                'L' => Dir::L,
                'R' => Dir::R,
                _ => unreachable!(),
            };
            let count = line[1..].parse::<usize>().unwrap();

            Move { dir, count }
        })
        .collect();
    println!("{:?}", moves);
}

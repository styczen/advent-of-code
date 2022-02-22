fn main() {
    // // Reference solution from https://github.com/timvisee/advent-of-code-2021/blob/master/day08a/src/main.rs
    // println!(
    //     "{}",
    //     include_str!("../../input_day_8")
    //         .lines()
    //         .flat_map(|l| l.split_once('|').unwrap().1.split_ascii_whitespace())
    //         .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7))
    //         .count()
    // );

    // My solution, done in not very nice way because I'm a noobie in Rust right now.
    let mut cnt = 0;
    let data = include_str!("../../input_day_8");
    for line in data.lines() {
        let digits = line.split("|").last().unwrap();
        for digit in digits.split_whitespace() {
            if matches!(digit.len(), 2 | 3 | 4 | 7) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}

fn split_line_to_numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn is_report_safe(nums: &Vec<i32>) -> bool {
    let correct_diffs = nums.windows(2).all(|pair| {
        let diff = (pair[0] - pair[1]).abs();
        diff >= 1 && diff <= 3
    });
    let is_increasing = nums.windows(2).all(|p| p[1] > p[0]);
    let is_decreasing = nums.windows(2).all(|p| p[1] < p[0]);
    return correct_diffs && (is_increasing || is_decreasing);
}

fn main() {
    println!("Day 2");
    println!("Part 1");
    let content = aoc_2024::load_file("inputs/02");
    let safe_lines: u64 = content
        .lines()
        .map(|line| is_report_safe(&split_line_to_numbers(line)) as u64)
        .sum();
    println!("{}", safe_lines);
    println!("Part 2");
    let safe_lines_p2: u64 = content
        .lines()
        .map(|line| {
            let nums = split_line_to_numbers(line);
            if is_report_safe(&nums) {
                1 as u64
            } else {
                let safe = (0..nums.len()).any(|i| {
                    let mut nums_cp = nums.clone();
                    nums_cp.remove(i);
                    is_report_safe(&nums_cp)
                });
                safe as u64
            }
        })
        .sum();
    println!("{}", safe_lines_p2);
}

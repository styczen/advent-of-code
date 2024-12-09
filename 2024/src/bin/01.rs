use std::collections::HashMap;

fn main() {
    println!("Day 1");
    println!("Part 1");
    let content = aoc_2024::load_file("inputs/01");
    let lines = content.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();
    let mut first = vec![0u64; lines_len];
    let mut second = vec![0u64; lines_len];
    for (i, ele) in lines.iter().enumerate() {
        let nums = ele.split_once(" ").unwrap();
        first[i] = nums.0.trim().parse::<u64>().unwrap();
        second[i] = nums.1.trim().parse::<u64>().unwrap();
    }
    first.sort();
    second.sort();
    let mut diffs_sum = 0;
    for (&f, &s) in std::iter::zip(&first, &second) {
        diffs_sum += (f as i64).abs_diff(s as i64);
    }
    println!("{}", diffs_sum);

    println!("Part 2");
    let mut first_cnt: HashMap<u64, u64> = HashMap::new();
    for num in first {
        first_cnt.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut second_cnt: HashMap<u64, u64> = HashMap::new();
    for num in second {
        second_cnt.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut cnt_sum = 0;
    for (num, cnt) in first_cnt {
        let scnt = match second_cnt.get(&num) {
            Some(&scnt) => scnt,
            None => 0,
        };
        cnt_sum += scnt * num * cnt
    }
    println!("{}", cnt_sum);
}

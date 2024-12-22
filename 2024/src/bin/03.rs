fn calculate_mul_sums(content: &str) -> u64 {
    let mut sum = 0u64;
    for m in content.split("mul(") {
        for n in m.split(")") {
            if let Some((l_str, r_str)) = n.split_once(',') {
                sum += match (l_str.parse::<u64>(), r_str.parse::<u64>()) {
                    (Ok(l), Ok(r)) => l * r,
                    _ => 0,
                };
            }
        }
    }
    sum
}

fn calculate_enabled_mul_sums(content: &str) -> u64 {
    let mut sum = 0u64;

    let mut enabled = true;

    // TODO

    sum
}

fn main() {
    println!("Day 3");
    println!("Part 1");
    let content = aoc_2024::load_file("inputs/03");
    let sum = calculate_mul_sums(&content);
    println!("{}", sum);

    println!("Part 2");
    let enabled_sum = calculate_enabled_mul_sums(&content);
    println!("{}", enabled_sum);
}

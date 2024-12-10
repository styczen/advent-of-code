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

fn main() {
    println!("Day 3");
    println!("Part 1");
    let content = aoc_2024::load_file("inputs/03_test");
    println!("{}", content);
    let sum = calculate_mul_sums(&content);
    println!("{}", sum);
    println!("Part 2");
    // TODO: Extract substrings between "do" and "dont" (in that order) 
    // and calculate sums for those substrings. Handle first split 
    // between start of the string and "do" or "dont" because all
    // muls there are enabled.

    let does: Vec<_> = content.split("do()").collect();
    let first = does.first();

    for m in content.split("do()") {
        println!("DO: {}", m);
        for n in m.split("don't()") {
            println!("DONT: {}", n);
        }
    }
}

use std::str::FromStr;

#[derive(Debug)]
struct BatteriesBank {
    joltages: Vec<u8>,
}

#[derive(Debug)]
struct ParseBatteriesBankError;

impl FromStr for BatteriesBank {
    type Err = ParseBatteriesBankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteriesBank {
            joltages: s
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or(ParseBatteriesBankError)
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl BatteriesBank {
    fn calculate_max_joltage(&self) -> u8 {
        let mut best_a = self.joltages[0];
        let mut max_joltage = 0;
        for i in 1..self.joltages.len() {
            let candidate = best_a * 10 + self.joltages[i];
            max_joltage = max_joltage.max(candidate);
            best_a = best_a.max(self.joltages[i]);
        }
        return max_joltage;
    }

    /// Calculate maximum joltage by building resulting number by removing
    fn calculate_max_joltage_by_n_removals(&self, k: usize) -> u64 {
        let mut stack: Vec<u8> = vec![];
        let mut removals = self.joltages.len() - k;
        for j in self.joltages.iter() {
            while !stack.is_empty() && *j > *stack.last().unwrap() && removals > 0 {
                stack.pop().unwrap();
                removals -= 1;
            }
            stack.push(*j);
        }
        value_from_array(&stack[..k])
    }

    /// Calculate maximum joltage by building resulting number iteratively
    fn calculate_max_joltage_by_n(&self, k: usize) -> u64 {
        let mut max_j_v: Vec<u8> = vec![];
        max_j_v.reserve(k);
        let mut start = 0;
        for i in 0..k {
            let remaining = k - i - 1;
            let end = self.joltages.len() - remaining - 1;
            let (best, cm) = self.joltages[start..=end].iter().enumerate().fold(
                (0, &self.joltages[start]),
                |(best_idx, best_val), (idx, val)| {
                    if val > best_val {
                        (idx, val)
                    } else {
                        (best_idx, best_val)
                    }
                },
            );
            max_j_v.push(*cm);
            start += best + 1;
        }
        max_j_v.shrink_to_fit();

        value_from_array(&max_j_v[..k])
    }
}

fn value_from_array(arr: &[u8]) -> u64 {
    let mut value: u64 = 0;
    for i in 0..arr.len() {
        value += arr[i] as u64 * 10u64.pow((arr.len() - 1 - i) as u32);
    }
    value
}

fn main() {
    let raw_data = std::fs::read_to_string("inputs/03").unwrap();
    let s = raw_data
        .lines()
        .map(|l| l.parse::<BatteriesBank>().unwrap())
        .map(|bb| bb.calculate_max_joltage())
        .fold(0usize, |acc, x| acc + x as usize);
    println!("Day 03 (part 1): {:?}", s);
    let s = raw_data
        .lines()
        .map(|l| l.parse::<BatteriesBank>().unwrap())
        .map(|bb| bb.calculate_max_joltage_by_n_removals(12))
        .fold(0u64, |acc, x| acc + x);
    println!("Day 03 (part 2): {:?}", s);
    let s = raw_data
        .lines()
        .map(|l| l.parse::<BatteriesBank>().unwrap())
        .map(|bb| bb.calculate_max_joltage_by_n(12))
        .fold(0u64, |acc, x| acc + x);
    println!("Day 03 (part 2): {:?}", s);
}

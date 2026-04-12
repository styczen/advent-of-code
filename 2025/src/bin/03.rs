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
}

fn main() {
    let raw_data = std::fs::read_to_string("inputs/03").unwrap();
    let s = raw_data
        .lines()
        .map(|l| l.parse::<BatteriesBank>().unwrap())
        .map(|bb| bb.calculate_max_joltage())
        .fold(0usize, |acc, x| acc + x as usize);
    println!("Day 03 (part 1): {:?}", s);
}

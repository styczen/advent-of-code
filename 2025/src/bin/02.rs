use std::str::FromStr;

#[derive(Debug)]
struct Range {
    low: u64,
    high: u64,
}

#[derive(Debug)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s.trim().split_once("-").ok_or(ParseRangeError)?;
        Ok(Range {
            low: low.parse::<_>().map_err(|_| ParseRangeError)?,
            high: high.parse::<_>().map_err(|_| ParseRangeError)?,
        })
    }
}

impl Range {
    fn get_repetitions(&self) -> Vec<u64> {
        let mut repetitions = Vec::new();
        for i in self.low..=self.high {
            let i_s = i.to_string();
            let l = i_s.len();
            if l % 2 == 0 {
                let (f, s) = i_s.split_at(l / 2);
                if f == s {
                    repetitions.push(i);
                }
            }
        }
        return repetitions;
    }
}

fn main() {
    let raw_data = std::fs::read_to_string("inputs/02").unwrap();
    let s = raw_data
        .split(",")
        .map(|p| p.parse::<Range>().unwrap())
        .flat_map(|r| r.get_repetitions())
        .fold(0, |acc, x| acc + x);
    println!("Day 02 (part 1): {}", s);
}

use std::str::FromStr;

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

#[derive(Debug)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir_raw = s.chars().next().ok_or(ParseMoveError)?;
        let dir = match dir_raw {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => unreachable!(),
        };
        let count = s[1..].parse::<usize>().map_err(|_| ParseMoveError)?;

        Ok(Move { dir, count })
    }
}

fn main() {
    let raw_data = std::fs::read_to_string("inputs/01").unwrap();
    let mut dial_value: isize = 50;
    let mut cross_zero_count: usize = 0;
    raw_data
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|m| {
            match m.dir {
                Dir::R => dial_value += m.count as isize,
                Dir::L => dial_value -= m.count as isize,
            }

            if dial_value % 100 == 0 {
                cross_zero_count += 1;
            }
        });
    println!("Day 01 (part 1): {}", cross_zero_count);

    dial_value = 50;
    cross_zero_count = 0;
    raw_data
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        .for_each(|m| {
            // compute first click index k (1..100) that will hit 0 during this move
            let k: isize = match m.dir {
                Dir::R => {
                    if dial_value.rem_euclid(100) == 0 {
                        100
                    } else {
                        100 - dial_value.rem_euclid(100)
                    }
                }
                Dir::L => {
                    if dial_value.rem_euclid(100) == 0 {
                        100
                    } else {
                        dial_value.rem_euclid(100)
                    }
                }
            };

            // count hits contributed by this rotation
            let hits: isize = if (m.count as isize) < k {
                0
            } else {
                1 + ((m.count as isize) - k) / 100
            };

            cross_zero_count += hits as usize;

            // update canonical position for next move
            dial_value = match m.dir {
                Dir::R => (dial_value + (m.count as isize)).rem_euclid(100),
                Dir::L => (dial_value - (m.count as isize)).rem_euclid(100),
            };
        });
    println!("Day 01 (part 2): {}", cross_zero_count);
}

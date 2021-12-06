#[derive(Debug)]
struct LanternfishPopulation {
    fish_per_reproduction_timer: [usize; 9],
}

impl LanternfishPopulation {
    fn advance_days(self: &mut Self, days: usize) {
        for _ in 0..days {
            let fish_on_0 = self.fish_per_reproduction_timer[0];
            for i in 0..6 {
                self.fish_per_reproduction_timer[i] = self.fish_per_reproduction_timer[i + 1];
            }
            self.fish_per_reproduction_timer[6] = fish_on_0 + self.fish_per_reproduction_timer[7];
            self.fish_per_reproduction_timer[7] = self.fish_per_reproduction_timer[8];
            self.fish_per_reproduction_timer[8] = fish_on_0;
        }
    }

    fn total_population(self: &Self) -> usize {
        self.fish_per_reproduction_timer.iter().sum()
    }
}

fn parse(input: &str) -> Option<LanternfishPopulation> {
    let fish: Vec<usize> = input
        .trim()
        .split(",")
        .map(|n| n.parse().ok())
        .collect::<Option<Vec<_>>>()?;
    let mut fish_per_reproduction_timer: [usize; 9] = [0; 9];
    for fish in fish {
        fish_per_reproduction_timer[fish] += 1;
    }
    Some(LanternfishPopulation {
        fish_per_reproduction_timer: fish_per_reproduction_timer,
    })
}

fn main() -> Result<(), ()> {
    let input = include_str!("../input");
    let mut parsed = parse(input).ok_or(())?;
    parsed.advance_days(256);
    let result = parsed.total_population();
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3,4,3,1,2";
        let mut parsed = parse(input).unwrap();
        parsed.advance_days(256);
        let result = parsed.total_population();
        assert_eq!(result, 26984457539);
    }
}

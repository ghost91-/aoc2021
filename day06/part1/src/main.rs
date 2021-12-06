struct Lanternfish {
    reproduction_timer: i32,
    children: Vec<Lanternfish>,
}

impl Lanternfish {
    fn advance_to_next_day(self: &mut Self) {
        for child in &mut self.children {
            child.advance_to_next_day();
        }
        if self.reproduction_timer == 0 {
            self.children.push(Lanternfish {
                reproduction_timer: 8,
                children: vec![],
            });
            self.reproduction_timer = 6;
        } else {
            self.reproduction_timer -= 1;
        }
    }

    fn number_of_descendents(self: &Self) -> usize {
        self.children
            .iter()
            .map(|child| child.number_of_descendents() + 1)
            .sum()
    }
}

fn calculate_population_after_days(lanternfish: &mut Vec<Lanternfish>, days: i32) -> usize {
    for _ in 0..days {
        for fish in &mut *lanternfish {
            fish.advance_to_next_day();
        }
    }
    lanternfish
        .iter()
        .map(|fish| fish.number_of_descendents() + 1)
        .sum()
}

fn parse(input: &str) -> Option<Vec<Lanternfish>> {
    input
        .trim()
        .split(",")
        .map(|n| {
            Some(Lanternfish {
                reproduction_timer: n.parse().ok()?,
                children: vec![],
            })
        })
        .collect()
}

fn main() -> Result<(), ()> {
    let input = include_str!("../input");
    let mut parsed = parse(input).ok_or(())?;
    let result = calculate_population_after_days(&mut parsed, 80);
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
        let result = calculate_population_after_days(&mut parsed, 80);
        assert_eq!(result, 5934);
    }
}

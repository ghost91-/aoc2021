#[derive(Debug)]
enum CrabError {
    ParseCrab(std::num::ParseIntError),
    CrabAmount,
    PositionDifference,
}

impl From<std::num::ParseIntError> for CrabError {
    fn from(e: std::num::ParseIntError) -> Self {
        CrabError::ParseCrab(e)
    }
}

fn parse(input: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    input.trim().split(",").map(|n| n.parse()).collect()
}

fn calculate_fuel_consumption(crabs: &Vec<usize>, position: usize) -> usize {
    crabs
        .iter()
        .map(|crab| {
            let steps = if *crab > position {
                *crab - position
            } else {
                position - *crab
            };
            steps * (steps + 1) / 2
        })
        .sum()
}

fn calculate_least_used_fuel(crabs: &Vec<usize>) -> Result<usize, CrabError> {
    let min = crabs.iter().min().ok_or(CrabError::CrabAmount)?;
    let max = crabs.iter().max().ok_or(CrabError::CrabAmount)?;

    (*min..(max + 1))
        .map(|position| calculate_fuel_consumption(crabs, position))
        .min()
        .ok_or(CrabError::PositionDifference)
}

fn main() -> Result<(), CrabError> {
    let input = include_str!("../input");
    let parsed = parse(input)?;
    let result = calculate_least_used_fuel(&parsed)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let parsed = parse(input).unwrap();
        let result = calculate_least_used_fuel(&parsed).unwrap();
        assert_eq!(result, 168);
    }
}

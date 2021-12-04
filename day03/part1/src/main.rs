fn parse(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .collect()
}

fn to_u32(slice: std::vec::Vec<u32>) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b)
}

fn calculate_power_consumption(diagnostic_report: Vec<Vec<u32>>) -> u32 {
    let length = diagnostic_report.len();
    let digit_sum = diagnostic_report
        .iter()
        .fold(vec![], |acc, e| match acc[..] {
            [] => e.clone(),
            _ => acc
                .iter()
                .zip(e.iter())
                .map(|pair| pair.0 + pair.1)
                .collect::<Vec<_>>(),
        });
    let most_common_digits: std::vec::Vec<u32> = digit_sum
        .iter()
        .map(|digit| if *digit as usize >= length / 2 { 1 } else { 0 })
        .collect();
    let least_common_digits = most_common_digits.iter().map(|e| 1 - e).collect();

    let gamma_rate = to_u32(most_common_digits);
    let epsilon_rate = to_u32(least_common_digits);

    gamma_rate * epsilon_rate
}

fn main() {
    let input = include_str!("../input");
    let diagnostic_report = parse(input).unwrap();
    let power_consumption = calculate_power_consumption(diagnostic_report);
    println!("{}", power_consumption);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let result = calculate_power_consumption(parse(input).unwrap());
        assert_eq!(result, 198);
    }
}

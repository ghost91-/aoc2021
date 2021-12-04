fn parse(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2))
                .collect::<Option<Vec<_>>>()
        })
        .collect()
}

fn to_u32(vec: Vec<u32>) -> u32 {
    vec.iter().fold(0, |acc, &b| acc * 2 + b)
}

fn calculate_rating(
    diagnostic_report: &Vec<Vec<u32>>,
    condition: fn(length: usize, column_sum: usize) -> bool,
    index: usize,
) -> Vec<u32> {
    let length = diagnostic_report.len();
    if length == 1 {
        return diagnostic_report[0].clone();
    }
    let column_sum = diagnostic_report
        .iter()
        .map(|line| line[index])
        .sum::<u32>() as usize;
    let filter_digit = if condition(length, column_sum) { 1 } else { 0 };
    calculate_rating(
        &diagnostic_report
            .iter()
            .filter(|line| line[index] == filter_digit)
            .map(|line| line.clone())
            .collect(),
        condition,
        index + 1,
    )
}

fn calculate_life_support_rating(diagnostic_report: Vec<Vec<u32>>) -> u32 {
    to_u32(calculate_rating(
        &diagnostic_report,
        |length, column_sum| length - column_sum <= column_sum,
        0,
    )) * to_u32(calculate_rating(
        &diagnostic_report,
        |length, column_sum| length - column_sum > column_sum,
        0,
    ))
}

fn main() {
    let input = include_str!("../input");
    let diagnostic_report = parse(input).unwrap();
    let life_support_rating = calculate_life_support_rating(diagnostic_report);
    println!("{}", life_support_rating);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let result = calculate_life_support_rating(parse(input).unwrap());
        assert_eq!(result, 230);
    }
}

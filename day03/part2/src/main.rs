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

fn calculate_oxygen_generator_rating(diagnostic_report: Vec<Vec<u32>>, index: usize) -> Vec<u32> {
    let length = diagnostic_report.len();
    if length == 1 {
        return diagnostic_report[0].clone();
    }
    let column_sum: u32 = diagnostic_report.iter().map(|line| line[index]).sum();
    let filter_digit = if (length - column_sum as usize) <= column_sum as usize {
        1
    } else {
        0
    };
    calculate_oxygen_generator_rating(
        diagnostic_report
            .iter()
            .filter(|line| line[index] == filter_digit)
            .map(|line| line.clone())
            .collect(),
        index + 1,
    )
}

fn calculate_co2_scrubber_rating(diagnostic_report: Vec<Vec<u32>>, index: usize) -> Vec<u32> {
    let length = diagnostic_report.len();
    if length == 1 {
        return diagnostic_report[0].clone();
    }
    let column_sum: u32 = diagnostic_report.iter().map(|line| line[index]).sum();
    let filter_digit = if length - column_sum as usize > column_sum as usize {
        1
    } else {
        0
    };
    calculate_co2_scrubber_rating(
        diagnostic_report
            .iter()
            .filter(|line| line[index] == filter_digit)
            .map(|line| line.clone())
            .collect(),
        index + 1,
    )
}

fn calculate_life_support_rating(diagnostic_report: Vec<Vec<u32>>) -> u32 {
    to_u32(calculate_oxygen_generator_rating(
        diagnostic_report.clone(),
        0,
    )) * to_u32(calculate_co2_scrubber_rating(diagnostic_report, 0))
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

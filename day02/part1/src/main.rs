use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines().collect()
}

fn parse(input: Vec<String>) -> Vec<(String, i32)> {
    input
        .iter()
        .map(|command| {
            let direction_and_amount: Vec<&str> = command.split(" ").collect();
            (
                direction_and_amount.first().unwrap().to_string(),
                direction_and_amount
                    .last()
                    .map(|i| i.parse::<i32>().unwrap())
                    .unwrap(),
            )
        })
        .collect()
}

fn calculate_final_position(input: Vec<(String, i32)>) -> i32 {
    let final_position = input.iter().fold((0, 0), |acc, e| match e.0.as_str() {
        "forward" => (acc.0 + e.1, acc.1),
        "down" => (acc.0, acc.1 + e.1),
        "up" => (acc.0, acc.1 - e.1),
        _ => acc,
    });
    final_position.0 * final_position.1
}

fn main() -> Result<(), Error> {
    let commands = read(File::open("input")?)?;
    let final_position = calculate_final_position(parse(commands));
    println!("{}", final_position);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let commands = parse(
            vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .iter()
            .map(|a| a.to_string())
            .collect(),
        );

        assert_eq!(calculate_final_position(commands), 150);
    }
}

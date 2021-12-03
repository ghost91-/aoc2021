enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
enum CommandError {
    ParseAmount(std::num::ParseIntError),
    ParseFormatting(String),
}

impl From<std::num::ParseIntError> for CommandError {
    fn from(e: std::num::ParseIntError) -> Self {
        CommandError::ParseAmount(e)
    }
}

impl std::str::FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[..] {
            [direction_string, amount_string] => {
                let amount = amount_string.parse::<i32>()?;
                match direction_string {
                    "forward" => Ok(Command::Forward(amount)),
                    "up" => Ok(Command::Up(amount)),
                    "down" => Ok(Command::Down(amount)),
                    _ => Err(CommandError::ParseFormatting("unknown direction".into())),
                }
            }
            _ => Err(CommandError::ParseFormatting("expected 2 arguments".into())),
        }
    }
}

fn parse(input: &str) -> Result<Vec<Command>, CommandError> {
    input
        .lines()
        .map(|command_string| command_string.parse::<Command>())
        .collect()
}

#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn calculate_final_position(commands: Vec<Command>) -> i32 {
    let final_position =
        commands
            .iter()
            .fold(Position::default(), |position, command| match command {
                Command::Forward(amount) => Position {
                    horizontal: position.horizontal + amount,
                    depth: position.depth + position.aim * amount,
                    ..position
                },
                Command::Up(amount) => Position {
                    aim: position.aim - amount,
                    ..position
                },
                Command::Down(amount) => Position {
                    aim: position.aim + amount,
                    ..position
                },
            });
    final_position.horizontal * final_position.depth
}

fn main() -> Result<(), CommandError> {
    let input = include_str!("../input");
    let final_position = calculate_final_position(parse(input)?);
    println!("{}", final_position);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let result = calculate_final_position(parse(input).unwrap());
        assert_eq!(result, 900);
    }
}

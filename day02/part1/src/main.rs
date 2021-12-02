enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|command| {
            let mut direction_and_amount = command.split_whitespace();
            let direction = direction_and_amount.next().expect("no direction");
            let amount = direction_and_amount
                .next()
                .expect("no amount")
                .parse::<i32>()
                .expect("amount is not an integer");
            match direction {
                "forward" => Command::Forward(amount),
                "up" => Command::Up(amount),
                "down" => Command::Down(amount),
                _ => panic!("unknown direction {}", direction),
            }
        })
        .collect()
}

#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
}

fn calculate_final_position(commands: Vec<Command>) -> i32 {
    let final_position =
        commands
            .iter()
            .fold(Position::default(), |position, command| match command {
                Command::Forward(amount) => Position {
                    horizontal: position.horizontal + amount,
                    ..position
                },
                Command::Up(amount) => Position {
                    depth: position.depth - amount,
                    ..position
                },
                Command::Down(amount) => Position {
                    depth: position.depth + amount,
                    ..position
                },
            });
    final_position.horizontal * final_position.depth
}

fn main() {
    let input = include_str!("../input");
    let final_position = calculate_final_position(parse(input));
    println!("{}", final_position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let result = calculate_final_position(parse(input));
        assert_eq!(result, 150);
    }
}

use std::collections::HashMap;

type Point = (i32, i32);
type Line = (Point, Point);

fn parse_pair<T>(s: &str, separator: &str, transformer: fn(s: &str) -> Option<T>) -> Option<(T, T)>
where
    T: Copy,
{
    let parts = s
        .split(separator)
        .map(|e| transformer(e))
        .collect::<Option<Vec<_>>>()?;
    match parts[..] {
        [first, last] => Some((first, last)),
        _ => None,
    }
}

fn parse_point(s: &str) -> Option<Point> {
    parse_pair(s, ",", |s| s.parse::<i32>().ok())
}

fn parse_line(line: &str) -> Option<Line> {
    parse_pair(line, " -> ", parse_point)
}

fn parse(input: &str) -> Option<Vec<Line>> {
    input.lines().map(parse_line).collect()
}

fn get_points_in_line(line: &Line) -> Vec<Point> {
    let x_direction = (line.1 .0 - line.0 .0).signum();
    let y_direction = (line.1 .1 - line.0 .1).signum();
    let mut points = vec![];
    let mut x = line.0 .0;
    let mut y = line.0 .1;
    while (x, y) != line.1 {
        points.push((x, y));
        x += x_direction;
        y += y_direction;
    }
    points.push(line.1);
    points
}

fn calculate_number_of_points_with_overlapping_lines(lines: &Vec<Line>) -> Option<usize> {
    let mut points_with_line_count = HashMap::new();
    for line in lines {
        for point in get_points_in_line(line) {
            *(points_with_line_count.entry(point).or_insert(0)) += 1;
        }
        // print_points_with_line_count(&points_with_line_count);
    }
    Some(
        points_with_line_count
            .iter()
            .filter(|(_, count)| **count > 1)
            .count(),
    )
}

fn print_points_with_line_count(points: &HashMap<Point, i32>) -> Option<()> {
    let max_x = points.iter().map(|((x, _), _)| x).max()?;
    let max_y = points.iter().map(|((_, y), _)| y).max()?;
    print!(" |");
    for x in 0..max_x + 1 {
        print!("{}", x);
    }
    println!();
    print!("-+");
    for _ in 0..max_x + 1 {
        print!("-");
    }
    println!();
    for y in 0..max_y + 1 {
        print!("{}|", y);
        for x in 0..max_x + 1 {
            match points.get(&(x, y)) {
                Some(count) => print!("{}", count),
                _ => print!("·"),
            }
        }
        println!();
    }
    println!();
    Some(())
}

fn main() -> Result<(), ()> {
    let input = include_str!("../input");
    let parsed = parse(input).ok_or(())?;
    let result = calculate_number_of_points_with_overlapping_lines(&parsed).ok_or(())?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
        let parsed = parse(input).unwrap();
        let result = calculate_number_of_points_with_overlapping_lines(&parsed).unwrap();
        assert_eq!(result, 12);
    }
}

use ndarray::{Array, Array2, ArrayView, Axis};

#[derive(Default, Clone)]
struct Cell {
    number: i32,
    marked: bool,
}

type Field = Array2<Cell>;

struct Bingo {
    field: Field,
    won: bool,
}

fn parse(input: &str) -> Option<(Vec<i32>, Vec<Bingo>)> {
    let mut blocks = input.split("\n\n");
    let drawn_numbers: Vec<i32> = blocks
        .next()?
        .split(",")
        .map(|number| number.parse())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let bingos: Vec<Bingo> = blocks
        .map(|block| {
            let mut field = Array::default((0, 5));
            for line in block.lines() {
                field
                    .push_row(ArrayView::from(
                        line.split_whitespace()
                            .map(|number| {
                                Ok::<Cell, std::num::ParseIntError>(Cell {
                                    number: number.parse()?,
                                    marked: false,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()
                            .ok()?
                            .as_slice(),
                    ))
                    .ok()?
            }
            Some(Bingo {
                field: field,
                won: false,
            })
        })
        .collect::<Option<_>>()?;
    Some((drawn_numbers, bingos))
}
fn main() -> Result<(), ()> {
    let input = include_str!("../input");
    let (ref drawn_numbers, ref mut bingos) = parse(input).ok_or(())?;
    let result = find_last_winning_bingo_score(drawn_numbers, bingos).ok_or(())?;
    println!("{}", result);
    Ok(())
}

fn is_bingo(field: &Field) -> bool {
    let found = field
        .axis_iter(Axis(0))
        .find(|a| a.iter().all(|cell| cell.marked))
        .or(field
            .axis_iter(Axis(1))
            .find(|a| a.iter().all(|cell| cell.marked)));
    match found {
        Some(_) => true,
        None => false,
    }
}

fn sum_of_unmarked_numbers(field: &Field) -> i32 {
    field
        .iter()
        .filter(|cell| !cell.marked)
        .map(|cell| cell.number)
        .sum()
}

fn find_last_winning_bingo_score(drawn_numbers: &Vec<i32>, bingos: &mut Vec<Bingo>) -> Option<i32> {
    let mut last_winning_bingo_score: Option<i32> = None;
    for number in drawn_numbers {
        for x in 0..5 {
            for y in 0..5 {
                for bingo in &mut *bingos {
                    if bingo.won {
                        continue;
                    }
                    if bingo.field[[x, y]].number == *number {
                        bingo.field[[x, y]].marked = true
                    }
                    if is_bingo(&bingo.field) {
                        bingo.won = true;
                        last_winning_bingo_score =
                            Some(sum_of_unmarked_numbers(&bingo.field) * number);
                    }
                }
            }
        }
    }
    last_winning_bingo_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";
        let (ref drawn_numbers, ref mut bingos) = parse(input).unwrap();
        let result = find_last_winning_bingo_score(drawn_numbers, bingos).unwrap();
        assert_eq!(result, 1924);
    }
}

use ndarray::{Array, Array2, ArrayView, Axis};

type Bingo = Array2<(i32, bool)>;

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
            let mut bingo = Array::default((0, 5));
            for line in block.lines() {
                bingo
                    .push_row(ArrayView::from(
                        line.split_whitespace()
                            .map(|number| {
                                Ok::<(i32, bool), std::num::ParseIntError>((number.parse()?, false))
                            })
                            .collect::<Result<Vec<_>, _>>()
                            .ok()?
                            .as_slice(),
                    ))
                    .ok()?
            }
            Some(bingo)
        })
        .collect::<Option<_>>()?;
    Some((drawn_numbers, bingos))
}
fn main() -> Result<(), ()> {
    let input = include_str!("../input");
    let (ref drawn_numbers, ref mut bingos) = parse(input).ok_or(())?;
    let result = find_first_winning_bingo_score(drawn_numbers, bingos).ok_or(())?;
    println!("{}", result);
    Ok(())
}

fn is_bingo(bingo: &Bingo) -> bool {
    let found = bingo
        .axis_iter(Axis(0))
        .find(|a| a.iter().all(|(_, marked)| *marked))
        .or(bingo
            .axis_iter(Axis(1))
            .find(|a| a.iter().all(|(_, maked)| *maked)));
    match found {
        Some(_) => true,
        None => false,
    }
}

fn sum_of_unmarked_numbers(bingo: &Bingo) -> i32 {
    bingo
        .iter()
        .filter(|(_, marked)| !marked)
        .map(|(number, _)| number)
        .sum()
}

fn find_first_winning_bingo_score(
    drawn_numbers: &Vec<i32>,
    bingos: &mut Vec<Bingo>,
) -> Option<i32> {
    for number in drawn_numbers {
        for x in 0..5 {
            for y in 0..5 {
                for bingo in &mut *bingos {
                    if bingo[[x, y]].0 == *number {
                        bingo[[x, y]].1 = true
                    }
                    if is_bingo(bingo) {
                        return Some(sum_of_unmarked_numbers(bingo) * number);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";
        let (ref drawn_numbers, ref mut bingos) = parse(input).unwrap();
        let result = find_first_winning_bingo_score(drawn_numbers, bingos).unwrap();
        assert_eq!(result, 4512);
    }
}

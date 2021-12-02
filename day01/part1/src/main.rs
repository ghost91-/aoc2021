use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let measurements = read(File::open("input")?)?;
    let increases = count_increases(measurements);
    println!("{}", increases);
    Ok(())
}

fn count_increases(measurements: Vec<i32>) -> usize {
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|pair| pair.1 > pair.0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let measurements: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increases(measurements), 7);
    }
}

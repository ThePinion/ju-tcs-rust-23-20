use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests;

pub fn head(path: &Path, count: usize) -> io::Result<Vec<String>> {
    let lines = read_lines(path)?;
    lines
        .into_iter()
        .take(count)
        .collect::<Vec<_>>()
        .into_iter()
        .collect()
}

pub fn tail(path: &Path, count: usize) -> io::Result<Vec<String>> {
    let lines = read_lines(path)?;
    Ok(lines
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<io::Result<Vec<_>>>()?
        .into_iter()
        .rev()
        .take(count)
        .rev()
        .collect())
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

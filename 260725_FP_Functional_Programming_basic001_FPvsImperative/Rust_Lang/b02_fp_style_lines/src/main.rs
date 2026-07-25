use std::{fs, io};

fn count_lines(file: &str) -> io::Result<usize> {
    let content = fs::read_to_string(file)?;
    Ok(content.lines().count())
}

fn main() -> io::Result<()> {
    let files = vec!["a.txt", "b.txt"];

    let total_lines: usize = files
        .iter()
        .map(|file| count_lines(file))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    println!("total lines : {total_lines} .");

    Ok(())
}

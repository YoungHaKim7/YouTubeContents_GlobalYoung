use std::{fs, io};

fn main() -> io::Result<()> {
    let files = vec!["a.txt", "b.txt"];

    let total_lines = files
        .iter()
        .map(|file| fs::read_to_string(file))
        .map(|result| result.map(|content| content.lines().count()))
        .try_fold(0usize, |total, count| count.map(|count| total + count))?;

    println!("total lines : {total_lines} ..");

    Ok(())
}

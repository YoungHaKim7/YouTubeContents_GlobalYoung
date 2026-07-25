use std::{fs, io};

fn main() -> io::Result<()> {
    println!("Imperative style");
    let files = vec!["a.txt", "b.txt"];

    let mut total_lines = 0;
    for file in files {
        let content = fs::read_to_string(file)?;
        total_lines += content.lines().count();
    }

    println!("total Line : {} .", total_lines);

    Ok(())
}

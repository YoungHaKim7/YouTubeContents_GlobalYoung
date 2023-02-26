fn count_chars(s: &str) -> usize {
    s.chars().map(|_| 1).sum()
}

fn main() {
    let emojis = "🗓🜁∈🌏";

    unsafe {
        println!("emoji : {}", emojis.get_unchecked(0..4));
        println!("emoji : {}", emojis.get_unchecked(4..7));
        println!("emoji : {}", emojis.get_unchecked(7..11));
    }

    println!(
        "char count: {}",
        count_chars(unsafe { emojis.get_unchecked(0..7) })
    );
}

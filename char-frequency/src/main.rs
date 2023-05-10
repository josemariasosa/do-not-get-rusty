use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    let mut characters: HashMap<char, u32> = HashMap::new();

    print!("Please enter some text: ");
    io::stdout().flush().unwrap();  // flush the output to ensure the print! message appears before read_line

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_ascii_lowercase().to_string();
    for ch in input.chars() {
        characters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut sorted_word_counts: Vec<(&char, &u32)> = characters.iter().collect();
    sorted_word_counts.sort_by(|a, b| b.1.cmp(a.1));

    for (key, value) in sorted_word_counts {
        println!("'{}': {}", key, value);
    }

    println!("You entered: {}", input.trim());
}
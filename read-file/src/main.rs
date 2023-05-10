use regex::Regex;
use std::collections::HashMap;
use std::{env, io, fs};

const MAX_CHAR_WORDS: usize = 15;
const MAX_FREQ: u32 = 50;

fn parse_args() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get(1) {
        Ok(arg.to_string())
    } else {
        Err(
            io::Error::new(
            io::ErrorKind::Other,
            "Argument not found!"
            )
        )
    }
}

fn read_file(path: &str) -> io::Result<String> {
    println!("The file you are analizing is: {:?}", path);
    fs::read_to_string(path)
}

fn generate_words_vector(text: &str) -> Vec<String> {
    let re = Regex::new(r"[a-zA-Z]+").unwrap();
    let words: Vec<_> = re.find_iter(text).map(
        |mat| mat.as_str().to_ascii_lowercase().to_string()
    ).collect();
    words
}

fn generate_frequency_vec(words: &Vec<String>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for word in words {
        *map.entry(word.to_owned()).or_insert(0) += 1;
    }
    map
}

/// This function normalize the frequency in order to make it fit between 1 and MAX_FREQ.
fn normalize_frequency(map: HashMap<String, u32>) -> HashMap<String, u32> {
    let mut map = map;
    let max_value = map.values().cloned().max().unwrap_or(0);
    for value in map.values_mut() {
        *value = *value * MAX_FREQ / max_value;
    }
    map
}

fn print_words_frequency_plot(freq_map: &HashMap<String, u32>, normal_map: &HashMap<String, u32>) {
    let mut vec: Vec<_> = freq_map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));

    for (key, value) in vec {
        let trimmed: String = key.chars().take(10).collect();
        println!(
            "{:>width$} - {:03} - {}",
            trimmed,
            value,
            "*".repeat(*normal_map.get(key).unwrap() as usize),
            width = MAX_CHAR_WORDS
        );
    }
}

fn main() -> io::Result<()> {
    let file_name = parse_args()?;
    let file_string = read_file(&file_name)?;

    let words_vec = generate_words_vector(&file_string);
    let freq_map = generate_frequency_vec(&words_vec);
    let normal_map = normalize_frequency(freq_map.clone());

    print_words_frequency_plot(&freq_map, &normal_map);
    Ok(())
}

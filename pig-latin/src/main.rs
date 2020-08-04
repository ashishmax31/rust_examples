const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

use std::io;

fn main() {
    loop {
        let user_input = {
            println!("Please enter a line of text: ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Expected a line of text");
            String::from(input.trim())
        };
        let result: Vec<String> = user_input
            .split_whitespace()
            .map(|word| pig_latinize(word))
            .collect();
        println!("Pig latin: {:?}\n", result.join(" "));
    }
}

fn pig_latinize(word: &str) -> String {
    let chars = word.chars().collect::<Vec<char>>();
    let (_, rem) = chars.split_at(1);
    match chars.first() {
        Some(c) => {
            if VOWELS.contains(c) {
                format!("{}-hay", word)
            } else {
                format!("{}-{}ay", construct_string(rem), c)
            }
        }
        None => panic!("Empty sentence!!"),
    }
}

fn construct_string(input: &[char]) -> String {
    input.iter().fold(String::new(), |mut acc, item| {
        acc.push(*item);
        acc
    })
}

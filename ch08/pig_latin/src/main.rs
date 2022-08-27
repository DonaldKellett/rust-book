use std::io;

fn main() {
    let stdin = io::stdin();
    let mut sentence = String::new();
    println!("Enter a sentence and we'll translate it to pig latin!");
    stdin
        .read_line(&mut sentence)
        .expect("The operating system should be able to read user input");
    println!("{}", to_pig_latin(&sentence));
}

fn to_pig_latin(sentence: &str) -> String {
    let vowels = "aeiouAEIOU";
    sentence
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|word| {
            let codepoints: Vec<_> = word.chars().collect();
            if vowels
                .chars()
                .collect::<Vec<_>>()
                .iter()
                .any(|&vowel| vowel == codepoints[0])
            {
                format!("{}-hay", word)
            } else {
                format!(
                    "{}-{}ay",
                    &codepoints[1..].iter().collect::<String>(),
                    codepoints[0]
                )
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let mut s = "hello";
    println!("{} first CharType is {:?}", s, first_char_type(&s));
    println!("{} to pig latin: {}", s, to_pig_latin(s));
    s = "ahoy";
    println!("{} first CharType is {:?}", s, first_char_type(&s));
    println!("{} to pig latin: {}", s, to_pig_latin(s));
}

#[derive(Debug)]
enum CharType {
    Vowel,
    Consonant,
}

use CharType::*;

fn first_char_type(s: &str) -> CharType {
    match s.chars().next().unwrap().to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => Vowel,
        _ => Consonant,
    }
}

fn to_pig_latin(s: &str) -> String {
    let mut pl = s.to_string();
    match first_char_type(s) {
        Vowel => pl += "-hay",
        Consonant => {
            let first_char = pl.remove(0);
            pl = format!("{}-{}{}", pl, first_char, "ay");
        }
    }
    pl
}


#[path = "../pattern_parse.rs"]
mod pattern_parse;

//Traits
use pattern_parse::ParsePattern;

pub fn run(input: std::string::String) {
    let valid_passwords = input.lines().filter( |s| {
        
        let parts = s.parse_pttrn("%u-%u %c: %s");
        let low = parts[0].uSize().unwrap() - 1;
        let high = parts[1].uSize().unwrap() - 1;
        let letter = parts[2].char().unwrap();
        let password = parts[3].string().unwrap();

        //Part 1
        //let count = password.matches(letter).count();
        //return low <= count && count <= high;

        //Part 2
        let characters: Vec<char> = password.chars().collect();
        return (characters[low] == letter || characters[high] == letter) && 
                !(characters[low] == letter && characters[high] == letter)
    });

    println!("{}", valid_passwords.count())
}
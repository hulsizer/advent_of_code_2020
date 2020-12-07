use std::collections::HashSet;
use std::collections::HashMap;

pub fn run(input: std::string::String) {

    //Part 1
    // let mut answer = 0;
    // let values = input.split("\n\n").map(|group| {
    //     let mut seats: HashSet<char> = HashSet::new();
    //     group.replace("\n", "").chars().for_each(|answer| {
    //         seats.insert(answer);
    //         return;
    //     });

    //     return seats.len();
    // }).for_each(|count| {
    //     answer += count;
    // });

    // println!("Answer: {}", answer)

    //Part 2
    let mut answer = 0;
    input.split("\n\n").map(|group| {
        
        let mut seats = HashMap::new();

        let people = group.matches("\n").count() + 1;

        group.replace("\n", "").chars().for_each(|answer| {
            *seats.entry(answer).or_insert(0) += 1;
            return;
        });

        return seats.iter().filter(|value| {
                    return *value.1 == people;
                }).count();
    }).for_each(|count| {
        answer += count;
    });

    println!("Answer: {}", answer)
}
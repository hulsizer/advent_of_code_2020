use std::collections::VecDeque;

pub fn run(input: std::string::String) {

    pt2(input);
}

fn pt1(input: std::string::String) {
    let mut preamble_values: VecDeque<i64> = input.lines().take(25).map(|value| value.parse::<i64>().unwrap()).collect();
    let mut found = false;

    input.lines().skip(25).for_each(|str_value| {
        if found { return; }

        let value = str_value.parse::<i64>().unwrap();
        
        let contained: Vec<i64> = preamble_values.iter().map(|preamble_value| {
            return (preamble_value - value).abs();
        }).filter(|new_value| {
            return preamble_values.contains(new_value);
        }).collect();

        if contained.len() == 0 {
            println!("Answer pt1 {:?}", value);
            found = true;
        }

        preamble_values.pop_front();
        preamble_values.push_back(value);
    });
}

fn pt2(input: std::string::String) {
    let goal = 400480901;
    let values: Vec<i64> = input.lines().map(|value| value.parse::<i64>().unwrap()).collect();
    let mut contigious_values = VecDeque::<i64>::new();

    'outer: for value in values {
        if value > goal {
            contigious_values = VecDeque::<i64>::new();
        }

        contigious_values.push_back(value);
        let mut run_down = contigious_values.clone();
        while run_down.len() >= 2 {
            let sum = run_down.iter().fold(0, |sum, x| sum + x);
            if sum == goal {
                
                let min = run_down.iter().min().unwrap();
                let max = run_down.iter().max().unwrap();

                println!("FOUND! {}", min + max);
                break 'outer;
            }
            run_down.pop_front();
        }
    }
}
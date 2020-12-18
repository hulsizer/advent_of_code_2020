use std::{collections::HashMap, vec};

use structopt::clap::value_t;


pub fn run(input: std::string::String) {
    pt2(input);
}

pub fn pt1(input: String) {

    let mut queue: Vec<i64> = input.split(",").map(|value| return value.parse::<i64>().unwrap()).collect();

    while queue.len() < 2020 {
        
        if let Some(position) = queue.iter().take(queue.len()-1).rev().position(|value| *value == queue[queue.len() - 1]) {
            queue.push((queue.len()-((queue.len()-1)-position)) as i64);
        } else {
            queue.push(0);
        }
    }

    println!("{:?}", queue[queue.len()-1]);
}

pub fn pt2(input: String) {

    let mut queue = vec![0;30000000];
    let mut value = 0;

    input.split(",").enumerate().for_each(|(index, str_value)| { 
        let result = str_value.parse::<i32>().unwrap();
        queue[value as usize] = index as i32;
        value = result;
    });
    for index in 7..30000000 {
        let v = queue[value as usize];
        let result = if v == 0 { 0 } else { index - v };
        queue[value as usize] = index;
        value = result;
    }
    
    println!("{:?}", value);
}
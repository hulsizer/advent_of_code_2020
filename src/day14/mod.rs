#[path = "../pattern_parse.rs"]
mod pattern_parse;

use std::collections::HashMap;
use std::collections::HashSet;

//Traits
use pattern_parse::ParsePattern;

pub fn run(input: String) {

    pt2(input);    
}

pub fn pt1(input: String) {
    let mut mem_addresses = HashMap::<usize, i64>::new();
    let mut or_mask: i64 = 0;
    let mut and_mask: i64 = 0;
    input.lines().for_each(|line| {
        match line[..2].as_ref()  {
            "ma" => {
                let parts = line.parse_pttrn("mask = %s");
                let mask = parts[0].string().unwrap();
                let mut new_or_mask: i64 = 0;
                let mut new_and_mask: i64 = i64::MAX;
                mask.chars().enumerate().for_each(|(index, char)| {
                    match char {
                        
                        '1' => {
                            new_or_mask |= 1 << (35 - index)
                        },
                        _ => ()
                    }
                });
                or_mask = new_or_mask;

                mask.chars().enumerate().for_each(|(index, char)| {
                    match char {
                        '0' => {
                            new_and_mask &= !(1 << (35 - index))
                        },
                        _ => ()
                    }
                });
                and_mask = new_and_mask;
            },
            "me" => {
                let parts = line.parse_pttrn("mem[%u] = %i");
                let mem_address = parts[0].uSize().unwrap();
                let value = parts[1].i64().unwrap();
        
                let mut final_value = value;
                
                final_value &= and_mask;
                final_value |= or_mask;
                mem_addresses.insert(mem_address, final_value);
            },
            _ => ()
        }
    });

    let value = mem_addresses.values().fold(0, |sum,value| return sum + *value);
    println!("{}", value);
}

pub fn pt2(input: String) {
    let mut mem_addresses = HashMap::<usize, i64>::new();
    let mut or_mask: i64 = 0;
    let mut x_mask: i64 = 0;
    let mut replacement_values = Vec::<i64>::new();
    
    input.lines().for_each(|line| {
        match line[..2].as_ref()  {
            "ma" => {
                let parts = line.parse_pttrn("mask = %s");
                let mask = parts[0].string().unwrap();
                let mut new_or_mask: i64 = 0;
                let mut new_x_mask: i64 = 0;
                let mut new_replacement_values = HashSet::<i64>::new();

                mask.chars().enumerate().for_each(|(index, char)| {
                    match char {
                        
                        '1' => {
                            new_or_mask |= 1 << (35 - index)
                        },
                        'X' => {
                            new_x_mask |= 1 << (35 - index);
                            
                            if new_replacement_values.is_empty() {
                                let value_1 = 1 << (35 - index);
                                let value_2 = 0 << (35 - index);
                                new_replacement_values.insert(value_1);
                                new_replacement_values.insert(value_2);
                            } else {
                                for value in new_replacement_values.clone().iter() {
                                    let value_1 = value | 1 << (35 - index);
                                    let value_2 = value | 0 << (35 - index);
                                    new_replacement_values.insert(value_1);
                                    new_replacement_values.insert(value_2);
                                }
                            }
                        },
                        _ => ()
                    }
                });
                or_mask = new_or_mask;
                x_mask = new_x_mask;
                replacement_values = new_replacement_values.into_iter().collect();
                replacement_values.sort();
            },
            "me" => {
                let parts = line.parse_pttrn("mem[%i] = %i");
                let mem_address = parts[0].i64().unwrap();
                let address_value = parts[1].i64().unwrap();
        
                let mut final_value = mem_address;
                final_value |= or_mask;
                for value in replacement_values.iter() {
                    let opposite_mask = final_value & !x_mask;
                    mem_addresses.insert((opposite_mask | value) as usize, address_value);
                }
            },
            _ => ()
        }
    });

    let value = mem_addresses.values().fold(0, |sum,value| return sum + *value);
    println!("{}", value);
}
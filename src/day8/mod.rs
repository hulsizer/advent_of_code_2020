use std::collections::HashSet;

pub fn run(input: std::string::String) {
    pt2(input);
}

pub fn pt1(input: std::string::String) {
    let mut accumulator: usize = 0;
    let mut instruction_set = HashSet::<usize>::new();
    let code: Vec<&str> = input.lines().collect();

    let mut instruction_row: usize = 0; 

    while instruction_row < code.len() {
        
        if instruction_set.contains(&instruction_row) {
            println!("Loop! Unwind");
            break;
        } else {
            instruction_set.insert(instruction_row);
        }

        let components: Vec<&str> = code[instruction_row].split(" ").collect();
        match components[0] {
            "acc" => {
                let mut chars = components[1].chars();
                match chars.next().unwrap() {
                    '+' => {
                        accumulator += chars.as_str().parse::<usize>().unwrap();
                    },
                    '-' => {
                        accumulator -= chars.as_str().parse::<usize>().unwrap();
                    }
                    _ => ()
                }
                instruction_row += 1;
            },
            "nop" => {
                instruction_row += 1;
            },
            "jmp" => {
                let mut chars = components[1].chars();
                match chars.next().unwrap() {
                    '+' => {
                        instruction_row += chars.as_str().parse::<usize>().unwrap();
                    },
                    '-' => {
                        instruction_row -= chars.as_str().parse::<usize>().unwrap();
                    }
                    _ => ()
                }
                
            },
            _ => ()
        }
    }
}

pub fn pt2(input: std::string::String) {
    let mut accumulator: usize = 0;
    let mut instruction_set = HashSet::<usize>::new();
    
    //From my understanding this is on the stack (&str)
    let mut code: Vec<String> = input.lines().map(|line| line.to_string() ).collect();

    let mut instruction_row: usize = 0; 
    let mut instruction_stack = Vec::<(usize, usize, HashSet::<usize>, String)>::new();

    let mut collect_operations = true;

    while instruction_row < code.len() {
        
        if instruction_set.contains(&instruction_row) {
            collect_operations = false;
            println!("Loop! Accumulator: {}", accumulator);

            match instruction_stack.pop() {
                Some(operation) => {
                    accumulator = operation.0;
                    instruction_row = operation.1;
                    instruction_set = operation.2;
                    
                    let components: Vec<String> = code[instruction_row].split(" ").map(|line| line.to_string() ).collect();
                    match components[0].as_str() {
                        "jmp" => { 
                            let together = format!("nop {}", components[1].as_str());
                            code[instruction_row] = together;
                        },
                        "nop" => { 
                            let together = format!("nop {}", components[1].as_str());
                            code[instruction_row] = together;
                        },
                        _ => (),
                    }
                },
                None => break
            }
        } else {
            instruction_set.insert(instruction_row);
        }

        let components: Vec<String> = code[instruction_row].split(" ").map(|line| line.to_string() ).collect();
        match components[0].as_str() {
            "acc" => {
                let mut chars = components[1].chars();
                match chars.next().unwrap() {
                    '+' => {
                        accumulator += chars.as_str().parse::<usize>().unwrap();
                    },
                    '-' => {
                        accumulator -= chars.as_str().parse::<usize>().unwrap();
                    }
                    _ => ()
                }
                instruction_row += 1;
            },
            "nop" => {
                if collect_operations {
                    instruction_stack.push((accumulator, instruction_row, instruction_set.iter().copied().collect(), components[1].clone()));
                }
                instruction_row += 1;
            },
            "jmp" => {
                if collect_operations {
                    instruction_stack.push((accumulator, instruction_row, instruction_set.iter().copied().collect(), components[1].clone()));
                }
                let mut chars = components[1].chars();
                match chars.next().unwrap() {
                    '+' => {
                        instruction_row += chars.as_str().parse::<usize>().unwrap();
                    },
                    '-' => {
                        instruction_row -= chars.as_str().parse::<usize>().unwrap();
                    }
                    _ => ()
                }
                
            },
            _ => ()
        }
    }

    println!("Accumulator {}", accumulator)
}
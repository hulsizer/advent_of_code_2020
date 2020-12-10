use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: std::string::String) {

    pt2(input);
}

pub fn pt1(input: std::string::String) {
    
    let adapters: HashSet::<i64> = input.lines()
                                    .map(|value| value.parse::<i64>().unwrap())
                                    .collect();

    let mut tree = HashMap::<i64, Vec<i64>>::new();
    for adapter in adapters.iter() {
        
        let mut nodes = Vec::<i64>::new();
        if adapters.contains(&(adapter-1)) {
            nodes.push(adapter-1);
        }
        else if adapters.contains(&(adapter-2)) {
            nodes.push(adapter-2);
        }
        else if adapters.contains(&(adapter-3)) {
            nodes.push(adapter-3);
        }

        tree.insert(*adapter, nodes);
    }

    let starting_node = adapters.iter().max().unwrap();
    let mut chain = traverse_pt1(&tree, *starting_node).unwrap_or_default();
    chain.insert(0, 0);
    println!("{:?}", chain);

    let mut one_difference_count = 0;
    let mut threes_difference_count = 0;

    for index in 0..chain.len()-1 {
        let current = chain[index];
        let next = chain[index + 1];
        
        if (current - next).abs() == 1 {
            one_difference_count += 1;
        }
        if (current - next).abs() == 3 {
            threes_difference_count += 1;
        }
    }
    
    println!("Answer: {}", one_difference_count * (threes_difference_count + 1))
}

pub fn traverse_pt1(lookup: &HashMap::<i64, Vec<i64>>, current: i64) -> Option<Vec::<i64>> {

    match lookup.get(&current) {
        Some(children) => {
            if children.is_empty() && current <= 3 {
                return Some(vec![current]);
            }
            for child in children {
                let chain = traverse_pt1(lookup, *child);
                if chain.is_some() {
                    let mut new_chain = chain.unwrap().clone();
                    new_chain.push(current);
                    return Some(new_chain);
                }
            }
        },
        _ => (),
    }
    return None
}

pub fn pt2(input: std::string::String) {

    let mut adapters: HashSet::<i64> = input.lines()
                                    .map(|value| value.parse::<i64>().unwrap())
                                    .collect();

    adapters.insert(0);
    let mut tree = HashMap::<i64, Vec<i64>>::new();
    for adapter in adapters.iter() {
        let mut nodes = Vec::<i64>::new();
        if adapters.contains(&(adapter-1)) {
            nodes.push(adapter-1);
        }
        if adapters.contains(&(adapter-2)) {
            nodes.push(adapter-2);
        }
        if adapters.contains(&(adapter-3)) {
            nodes.push(adapter-3);
        }
         
        tree.insert(*adapter, nodes);
    }
    let mut memoize = HashMap::<i64,i64>::new();
    let starting_node = adapters.iter().max().unwrap();
    let count = traverse_pt2(&tree, &mut memoize, *starting_node);
    println!("{:?}", count);
}

pub fn traverse_pt2(lookup: &HashMap::<i64, Vec<i64>>, memoize: &mut HashMap::<i64, i64>, current: i64) -> i64 {

    if current == 0 {
        memoize.insert(current, 1);
        return 1
    }

    match memoize.get(&current) {
        Some(value) => return *value,
        None => {
            match lookup.get(&current) {
                Some(children) => {
                    let sum = children.iter().fold(0, |sum, child| sum + traverse_pt2(lookup, memoize, *child));
                    memoize.insert(current, sum);
                    return sum
                },
                _ => (),
            }
            return 1
        }
    }   
}
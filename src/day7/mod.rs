use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: std::string::String) {
    let sanatized_input = input.replace("bags", "")
                                .replace("bag", "")
                                .replace(".", "")
                                .replace("no other", "");

    // Part 1                           
    // let mut tree = HashMap::<std::string::String, Vec<(usize, std::string::String)>>::new();
    // sanatized_input.lines().for_each(|line| {
    //     let components: Vec<&str> = line.trim().split("contain").collect();
    //     let key = components[0].trim();
    //     components[1].trim().split(",").for_each(|bag| {
    //         if bag == "" {
    //             return;
    //         }
    //         let components: Vec<&str> = bag.trim().split(" ").collect();
    //         let bag_type = components[1..].join(" ").to_owned();
            
    //         tree.entry(bag_type).or_insert(vec![]).push((components[0].trim().parse::<usize>().unwrap(), key.to_string()));
    //     });
    // });
    // let goal = "shiny gold";
    // let mut set = HashSet::<std::string::String>::new();

    // traverse_pt1(&tree, &mut set, goal);

    // println!("{:?}", set);
    // println!("{}", set.len() - 1)

    let mut tree = HashMap::<std::string::String, Vec<(usize, std::string::String)>>::new();
    sanatized_input.lines().for_each(|line| {
        let components: Vec<&str> = line.trim().split("contain").collect();
        let key = components[0].trim();
        components[1].trim().split(",").for_each(|bag| {
            if bag == "" {
                return;
            }
            let components: Vec<&str> = bag.trim().split(" ").collect();
            let bag_type = components[1..].join(" ").to_owned();
            
            tree.entry(key.to_string()).or_insert(vec![]).push((components[0].trim().parse::<usize>().unwrap(), bag_type));
        });
    });

    let goal = "shiny gold";
    let size = traverse_pt2(&tree, goal);
    println!("{}", size);
    
}

pub fn traverse_pt1(lookup: &HashMap::<std::string::String, Vec<(usize, std::string::String)>>, used_bags: &mut HashSet::<std::string::String>, current: &str) {

    used_bags.insert(current.to_string());
    match lookup.get(current) {
        Some(children) => {
            children.iter().for_each(|bag| {
                traverse_pt1(lookup, used_bags, bag.1.as_str());
            });
        },
        _ => (),
    }
}

pub fn traverse_pt2(lookup: &HashMap::<std::string::String, Vec<(usize, std::string::String)>>, current: &str) -> usize {

    let mut size = 0;
    match lookup.get(current) {
        Some(children) => {
            children.iter().for_each(|bag| {
                size += bag.0 + (bag.0 * traverse_pt2(lookup, bag.1.as_str()));
            });

            return size;
        },
        _ => return 0,
    }
    
}
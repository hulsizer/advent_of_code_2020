
pub fn run(input: std::string::String) {
    let mut numbers: Vec<i32> = input.lines().map( |s| s.parse::<i32>().unwrap() ).collect();
    numbers.sort();

    //Part 1
    if let Some((value_one, value_two)) = find_components(&numbers, 2020) {
        println!("Total Part 1: {}", numbers[value_one] * numbers[value_two]);
    }

    //Part 2
    for elem in 0..numbers.len() {
        if 2020 - numbers[elem] > 0 {
            if let Some((value_one, value_two)) = find_components(&numbers, 2020 - numbers[elem]) {
                println!("Total Part 2: {}", numbers[value_one] * numbers[value_two] * numbers[elem]);
                break;
            }
        }
    }
}

fn find_components(numbers: &Vec<i32>, value: i32) -> Option<(usize, usize)> {
    let mut low_iter = 0;
    let mut high_iter = numbers.len() - 1;
    while low_iter != high_iter {
        let total = numbers[low_iter] + numbers[high_iter];

        if total > value {
            high_iter = high_iter - 1;
        } else if total < value {
            low_iter = low_iter + 1;
        } else {
            return Some((low_iter, high_iter));
        }
    }
    return None
}
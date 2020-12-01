
pub fn run(input: std::string::String) {
    let mut numbers: Vec<i32> = input.lines().map( |s| s.parse::<i32>().unwrap() ).collect();
    numbers.sort();

    //Part 1
    let a = find_components(&numbers, 2020);
    println!("Total Part 1: {}", numbers[a.0] * numbers[a.1]);


    //Part 2
    for elem in 0..numbers.len() {
        if 2020 - numbers[elem] > 0 {
            let a = find_components(&numbers, 2020 - numbers[elem]);
            if a.0 != 0 || a.1 != 0 {
                println!("Total Part 2: {}", numbers[a.0] * numbers[a.1] * numbers[elem]);
                break;
            }
        }
    }
}

fn find_components(numbers: &Vec<i32>, value: i32) -> (usize, usize) {
    let mut low_iter = 0;
    let mut high_iter = numbers.len() - 1;
    while low_iter != high_iter {
        let total = numbers[low_iter] + numbers[high_iter];

        if total > value {
            high_iter = high_iter - 1;
        } else if total < value {
            low_iter = low_iter + 1;
        } else {
            return (low_iter, high_iter);
        }
    }
    return (0, 0)
}

pub fn run(input: std::string::String) {
    pt2(input);
}

pub fn pt1(input: String) {
    let mut lines: Vec<String> = input.lines().map(|str| str.to_string()).collect();
    let id = lines.remove(0).parse::<i32>().unwrap();
    let buses: Vec<i32> = lines.first().unwrap()
                     .split(",")
                     .map(|value| {
                         if value == "x" {
                             return -1;
                         }
                         return value.parse::<i32>().unwrap()
                     }).filter(|value| *value != -1)
                     .collect();


    let mut adjusted: Vec<(i32, i32)> = times.iter().map(|time| {
        return (((id as f32 / *time as f32).ceil() as i32 * *time) - id, *time);
    }).collect();

    adjusted.sort_by(|value_1, value_2| value_1.0.cmp(&value_2.0));

    let element = adjusted.first().unwrap();
    println!("Time to Wait: {}, Bus ID: {}, Puzzle Answer: {}", element.0, element.1, element.0 * element.1);
}

pub fn pt2(input: String) {

    let buses: Vec<(usize,usize)> = input.lines().skip(1).next().unwrap().split(',')
        .enumerate()
        .filter(|(_,v)| *v != "x")
        .map(|(i,c)| (i, c.parse::<usize>().unwrap()))
        .collect();

    let p2 = buses.iter()
        .fold((0,1), |(solution_so_far, product_so_far), (remainder, bus_id)| {
            
            println!("Start: {:?} Current: {:?}", (solution_so_far, product_so_far), (remainder, bus_id));
            
            let mut final_solution = solution_so_far;
            
            while (final_solution+remainder) % bus_id != 0 {
                final_solution += product_so_far;
                println!("Nothing: {:?}", final_solution);
            }
            
            (final_solution, product_so_far*bus_id)
        })
        .0;

       println!("{}", p2)
}

pub fn run(input: std::string::String) {

    let mut seats: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    loop {
        let fill_result = fill_seats(&seats);
        if !fill_result.0 { println!("No changes on Fill"); break };
        seats = fill_result.1;

        let unfill_result = unfill_seats(&seats);
        if !unfill_result.0 { println!("No changes on unfill"); break };
        seats = unfill_result.1;
    }
    

    let mut count = 0;
    seats.iter().for_each(|row| row.iter().for_each(|item| if *item == '#' { count += 1}));
    println!("{:?}", count);
}

pub fn check_slope(seats: &Vec<Vec<char>>, position: (usize, usize), slope: (i32, i32)) -> bool {
    let mut loop_index = 1;

    loop {
        let new_x: i32 = position.0 as i32 + (slope.0 * loop_index);
        let new_y: i32 = position.1 as i32 + (slope.1 * loop_index);

        if new_y >= seats.len() as i32 || new_y < 0 {
            break;
        }
        if new_x >= seats[new_y as usize].len() as i32 || new_x < 0 {
            break;
        }

        if seats[new_y as usize][new_x as usize] == 'L' {
            return false
        }

        if seats[new_y as usize][new_x as usize] == '#' {
            return true
        }
        loop_index += 1;
    }

    return false
}

pub fn fill_seats(seats: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {

    let mut new_seats = seats.clone();
    let mut changes = false;
    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            //let length = seats[y].len() - 1;
            //let y_length = seats.len() - 1;
            match seats[y][x] {
                'L' => {
                    let mut see_seat = false;
                    see_seat |= check_slope(&seats, (x, y), (-1,-1));
                    see_seat |= check_slope(&seats, (x, y), (0,-1));
                    see_seat |= check_slope(&seats, (x, y), (1,-1));
                    see_seat |= check_slope(&seats, (x, y), (1,0));
                    see_seat |= check_slope(&seats, (x, y), (1,1));
                    see_seat |= check_slope(&seats, (x, y), (0,1));
                    see_seat |= check_slope(&seats, (x, y), (-1,1));
                    see_seat |= check_slope(&seats, (x, y), (-1,0));

                    //Part 1
                    // let min_x: usize = std::cmp::max(0, (x as i32)-1) as usize;
                    // let max_x: usize = std::cmp::min(length, x+1);
                    // let min_y: usize = std::cmp::max(0, (y as i32)-1) as usize;
                    // let max_y: usize = std::cmp::min(y_length, y+1);
                    // let mut count = 0;

                    // for check_x in min_x..=max_x {
                    //     for check_y in min_y..=max_y {
                    //         if check_x == x && check_y == y {
                    //             continue;
                    //         }
                    //         if seats[check_y][check_x] == '#' {
                    //             count += 1;
                    //         }
                    //     }
                    // }

                    if !see_seat {
                        new_seats[y][x] = '#';
                        changes = true;
                    }
                }
                _ => ()
            }
        }
    }
    return (changes, new_seats);
}

pub fn unfill_seats(seats: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    
    let mut new_seats = seats.clone();
    let mut changes = false;
    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            //let length = seats[y].len() - 1;
            //let y_length = seats.len() - 1;
            match seats[y][x] {
                '#' => {

                    let mut count = 0;
                    count += if check_slope(&seats, (x, y), (-1,-1)) { 1 } else { 0 };
                    count += if check_slope(&seats, (x, y), (0,-1)) { 1 } else { 0 };
                    count += if check_slope(&seats, (x, y), (1,-1)) { 1 } else { 0 };
                    count += if check_slope(&seats, (x, y), (1,0)) { 1 } else { 0 };
                    count += if check_slope(&seats, (x, y), (1,1)) { 1 } else { 0 };
                    count += if check_slope(&seats, (x, y), (0,1)) { 1 } else { 0 };
                    count += if check_slope(&seats, (x, y), (-1,1)) { 1 } else { 0 };
                    count += if check_slope(&seats, (x, y), (-1,0)) { 1 } else { 0 };
            
                    //Part 1
                    // let min_x: usize = std::cmp::max(0, (x as i32)-1) as usize;
                    // let max_x: usize = std::cmp::min(length, x+1);
                    // let min_y: usize = std::cmp::max(0, (y as i32)-1) as usize;
                    // let max_y: usize = std::cmp::min(y_length, y+1);
                    // let mut count = 0;

                    // for check_x in min_x..=max_x {
                    //     for check_y in min_y..=max_y {
                    //         if check_x == x && check_y == y {
                    //             continue;
                    //         }
                    //         if seats[check_y][check_x] == '#' {
                    //             count += 1;
                    //         }
                    //     }
                    // }

                    //if count >= 4 {
                    if count >= 5 {
                        new_seats[y][x] = 'L';
                        changes = true;
                    }
                }
                _ => ()
            }
        }
    }
    return (changes, new_seats);
}


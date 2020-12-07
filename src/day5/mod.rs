use std::collections::HashSet;



pub fn run(input: std::string::String) {
    //Part 1
    // let values = input.lines().map(|line| {
    //     let mut low = 0;
    //     let mut high = 127;
    //     let mut lowColumn = 0;
    //     let mut highColumn = 7;
    //     line.chars().for_each(|char| {
    //         match char {
    //             'F' => high = ((high - low) / 2) + low,
    //             'B' => low = ((high - low) / 2) + 1 + low,
    //             'R' => lowColumn = ((highColumn - lowColumn) / 2) + 1 + lowColumn,
    //             'L' => highColumn = (highColumn - lowColumn) / 2  + lowColumn,
    //             _ => ()
    //         }

    //         println!("F: {}, B: {}, L: {}, R: {}", low, high, lowColumn, highColumn);
    //     });

    //     return (high * 8) + highColumn;
    // });
    // println!("{:?}", values.max())

    let seats: HashSet<i32> = (0..1024).collect();

    let seats_taken: HashSet<i32> = input.lines().map(|line| {
        let mut low = 0;
        let mut high = 127;
        let mut low_column = 0;
        let mut high_column = 7;
        line.chars().for_each(|char| {
            match char {
                'F' => high = ((high - low) / 2) + low,
                'B' => low = ((high - low) / 2) + 1 + low,
                'R' => low_column = ((high_column - low_column) / 2) + 1 + low_column,
                'L' => high_column = (high_column - low_column) / 2  + low_column,
                _ => ()
            }
        });

        return (high * 8) + high_column;
    }).collect();

    let mut vec = seats.difference(&seats_taken).into_iter().collect::<Vec<&i32>>();
    vec.sort();
    println!("Seats Open: {:?}", vec);
}
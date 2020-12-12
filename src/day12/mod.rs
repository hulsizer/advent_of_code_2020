
pub fn run(input: std::string::String) {
    pt2(input);
}

pub fn pt1(input: String) {
    let mut north_south = 0;
    let mut east_west = 0;
    let mut direction = 'E';
    input.lines().for_each(|line| {
        let mut line_string: String = line.to_string();
        let cmd = line_string.remove(0);
        let value = line_string.parse::<i32>().unwrap();
        match cmd {
            'F' => {
                match direction {
                    'N' => {
                        north_south += value;
                    }
                    'S' => {
                        north_south -= value;
                    }
                    'E' => {
                        east_west += value;
                    }
                    'W' => {
                        east_west -= value;
                    }
                    _ => ()
                }
            }
            'R' => {
                match direction {
                    'N' => {
                        match value {
                            90 => direction = 'E',
                            180 => direction = 'S',
                            270 => direction = 'W',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    'S' => {
                        match value {
                            90 => direction = 'W',
                            180 => direction = 'N',
                            270 => direction = 'E',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    'E' => {
                        match value {
                            90 => direction = 'S',
                            180 => direction = 'W',
                            270 => direction = 'N',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    'W' => {
                        match value {
                            90 => direction = 'N',
                            180 => direction = 'E',
                            270 => direction = 'S',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    _ => ()
                }
            }
            'L' => {
                match direction {
                    'N' => {
                        match value {
                            90 => direction = 'W',
                            180 => direction = 'S',
                            270 => direction = 'E',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    'S' => {
                        match value {
                            90 => direction = 'E',
                            180 => direction = 'N',
                            270 => direction = 'W',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    'E' => {
                        match value {
                            90 => direction = 'N',
                            180 => direction = 'W',
                            270 => direction = 'S',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    'W' => {
                        match value {
                            90 => direction = 'S',
                            180 => direction = 'E',
                            270 => direction = 'N',
                            _ => println!("Error Value: {}", value)
                        }
                    }
                    _ => ()
                }
            }
            'N' => {
                north_south += value;
            }
            'S' => {
                north_south -= value;
            }
            'E' => {
                east_west += value;
            }
            'W' => {
                east_west -= value;
            }
            _ => println!("Error: {}", cmd)
        }
    });

    println!("North/South: {}, East/West: {}, Facing: {}", north_south, east_west, direction);
}

pub fn pt2(input: String) {
    let mut north_south = 0;
    let mut east_west = 0;
    let mut direction = 'E';
    let mut distance = (0, 0);
    let mut waypoint = (10,1);

    input.lines().for_each(|line| {
        let mut line_string: String = line.to_string();
        let cmd = line_string.remove(0);
        let value = line_string.parse::<i32>().unwrap();
        
        match cmd {
            'F' => {
                distance.0 += waypoint.0 * value;
                distance.1 += waypoint.1 * value;
            }
            'R' => {
                let rotation = value / 90;
                for _ in 0..rotation {
                    waypoint = (waypoint.1, -waypoint.0)
                }
            }
            'L' => {
                let rotation = value / 90;
                for _ in 0..rotation {
                    waypoint = (-waypoint.1, waypoint.0)
                }
            }
            'N' => {
                waypoint.1 += value;
            }
            'S' => {
                waypoint.1 -= value;
            }
            'E' => {
                waypoint.0 += value;
            }
            'W' => {
                waypoint.0 -= value;
            }
            _ => println!("Error: {}", cmd)
        }
    });

    println!("North/South: {}, East/West: {}, Waypoint: {:?}", distance.1, distance.0, waypoint);
}
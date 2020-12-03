
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct Traversal {
    position: Point,
    slope: Point,
    trees_hit: usize,
}

impl Traversal {

    fn new(slope: Point) -> Traversal {
        return Traversal {
            position: Point { x: 0, y: 0},
            slope: slope,
            trees_hit: 0
        };
    }
    fn step(&mut self) {
        self.position.x += self.slope.x;
        self.position.y += self.slope.y;
    }
}

pub fn run(input: std::string::String) {
    
    //Part 1
    //let mut traversals = vec![Traversal::new(Point { x: 3, y: 1})];

    //Part 2
    let mut traversals = vec![Traversal::new(Point { x: 1, y: 1}),
                                           Traversal::new(Point { x: 3, y: 1}),
                                           Traversal::new(Point { x: 5, y: 1}),
                                           Traversal::new(Point { x: 7, y: 1}),
                                           Traversal::new(Point { x: 1, y: 2})];

    for (line_num,line) in input.lines().enumerate() {
        for traversal in traversals.iter_mut() {
            if line_num % traversal.slope.y == 0 {
                if line.chars().nth(traversal.position.x % line.len()).unwrap() == '#' {
                    traversal.trees_hit += 1;
                }

                traversal.step();
            }
        }
    }

    // Note: This type doesnt step . . . hmmmm
    // input.lines().enumerate().for_each( |(line_num, line)| {
    //     traversals.iter_mut()
    //         .filter(| traversal| line_num % traversal.slope.y == 0)
    //         .filter( | traversal| line.chars().nth(traversal.position.x % line.len()).unwrap() == '#')
    //         .for_each(| traversal| traversal.trees_hit += 1)
    // });

    let mut total = 1;
    for traversal in traversals {
        println!("Slope: {} -- Trees Hits: {}", traversal.slope, traversal.trees_hit);
        total *= traversal.trees_hit;
    }

    println!("Total {}", total);
}
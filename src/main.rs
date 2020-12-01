use structopt::StructOpt;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;


#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    match args.pattern.as_str() {
        "day1" => day1::run(content),
        "day2" => day2::run(content),
        "day3" => day3::run(content),
        "day4" => day4::run(content),
        "day5" => day5::run(content),
        "day6" => day6::run(content),
        "day7" => day7::run(content),
        "day8" => day8::run(content),
        "day9" => day9::run(content),
        "day10" => day10::run(content),
        "day11" => day11::run(content),
        "day12" => day12::run(content),
        "day13" => day13::run(content),
        "day14" => day14::run(content),
        "day15" => day15::run(content),
        "day16" => day16::run(content),
        "day17" => day17::run(content),
        "day18" => day18::run(content),
        "day19" => day19::run(content),
        "day20" => day20::run(content),
        "day21" => day21::run(content),
        "day22" => day22::run(content),
        "day23" => day23::run(content),
        "day24" => day24::run(content),
        "day25" => day25::run(content),
        _ => println!("No Day Found!")
    }
}

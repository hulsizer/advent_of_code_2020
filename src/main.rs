use structopt::StructOpt;

mod day1;

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
        _ => println!("No Day Found!")
    }
}

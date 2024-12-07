use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run (1-25), if not specified all implemented days will be run
    #[arg(short, long)]
    day: Option<u8>,

    /// Use sample input instead of real input
    #[arg(short, long)]
    sample: bool,
}

fn main() {
    let args = Args::parse();

    match args.day {
        Some(day) => {
            aoc2024::run_solution(day, args.sample);
        }
        None => {
            // Run all implemented solutions
            for day in 1..=25 {
                if let Some(_) = aoc2024::get_solution(day) {
                    aoc2024::run_solution(day, args.sample);
                }
            }
        }
    }
}

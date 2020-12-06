use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: [run] [problemNumber]");
        println!("Example: cargo run --release -- 1a");
    }

    let start = std::time::Instant::now();

    let out: String = match args.get(1).unwrap().as_str() {
        "1a" => day01::run_1a().to_string(),
        "1b" => day01::run_1b().to_string(),

        "2a" => day02::run_2a().to_string(),
        "2b" => day02::run_2b().to_string(),

        "3a" => day03::run_3a().to_string(),
        "3b" => day03::run_3b().to_string(),

        "4a" => day04::run_4a().to_string(),
        "4b" => day04::run_4b().to_string(),

        "5a" => day05::run_5a().to_string(),
        "5b" => day05::run_5b().to_string(),

        "6a" => day06::run_6a().to_string(),
        "6b" => day06::run_6b().to_string(),

        other => {
            format!("Unrecognized problem number {}", other)
        }
    };

    println!("{}", out);
    println!("Process took {:.5} seconds", start.elapsed().as_secs_f32());
}

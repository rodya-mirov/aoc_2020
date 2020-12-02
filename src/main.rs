use std::env;

mod day01;
mod day02;

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

        other => {
            format!("Unrecognized problem number {}", other)
        }
    };

    println!("{}", out);
    println!("Process took {:.3} seconds", start.elapsed().as_secs_f32());
}

use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <day_number> <part_number>", args[0]);
        return;
    }
    let day_number = args[1].parse::<u32>().unwrap();
    let part_number = args[2].parse::<u32>().unwrap();

    let output = match day_number {
        1 => day1::run("./inputs/day1/input.txt".to_string(), part_number),
        2 => day2::run("./inputs/day2/input.txt".to_string(), part_number),
        _ => {
            println!("Day {} not implemented yet", day_number);
            Err("Day not implemented yet".into())
        }
    };

    match output {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

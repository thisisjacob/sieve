use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 2 {
        println!("Invalid arguments.");
        std::process::exit(1);
    }

    // TODO: Check that limit > 2
    let limit = args[1].parse::<u32>().expect("Not a valid limit.");
    let nums = [2..limit + 1];
    let p = 2;
}

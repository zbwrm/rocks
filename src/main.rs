use rand::Rng;
use std::env::args;
use std::process;

fn main() {
    let args: Vec<String> = args().collect();
    let num_args = args.len();

    if num_args != 3 {
        println!("Not implemented yet! Sorry :)");
        process::exit(0);
    } else if &args[1] != "-i" {
        println!("No other CLI options yet, sorry :)");
        process::exit(0);
    }

    let dice: Vec<&str> = args[2].split("d").collect();

    let num: u16 = dice[0].trim().parse().unwrap();
    let size: u16 = dice[1].trim().parse().unwrap();

    let mut rng = rand::thread_rng();

    for d in 0..num {
        println!("d{} {}: {}", size, d + 1, rng.gen_range(1..size + 1));
    }
}

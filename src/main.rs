use clap::builder::{Arg, ArgGroup};
use clap::Command;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::process;

const CONTROL_CHARS: &[char] = &['d', '>'];

struct Die {
    num: usize,
    sides: u16,
}

impl Die {
    fn new(num: usize, sides: u16) -> Die {
        Die { num, sides }
    }

    fn roll(self, rng: &mut ThreadRng) -> Vec<u16> {
        let mut rolls = Vec::with_capacity(self.num);

        for _ in 0..self.num {
            rolls.push(rng.gen::<u16>() % self.sides);
        }

        rolls
    }
}

fn parse_dice(dice: &str, operators: &[char]) -> Vec<String> {
    // TODO: associate operators with their values
    // i.e. "2d6" -> "2","d6" not "2d","6"
    // use https://crates.io/crates/str_splitter?
    dice.split_inclusive(operators)
        .map(str::to_string)
        .collect()
}

fn main() {
    let m = Command::new("rx")
        .about("Rolls dice.")
        .arg(
            Arg::new("individual")
                .short('i')
                .help("Rolls dice individually")
                .num_args(0),
        )
        .arg(
            Arg::new("sum")
                .short('s')
                .help("Sums rolled dice to one value")
                .num_args(0),
        )
        .arg(
            Arg::new("macro")
                .short('m')
                .help("Rolls on a user-defined macro [NOT IMPLEMENTED YET]")
                .num_args(0),
        )
        .group(
            ArgGroup::new("mode")
                .id("mode")
                .args(["individual", "sum", "macro"])
                .required(true),
        )
        .arg(
            Arg::new("rolls")
                .short('r')
                .long("rolls")
                .help("Number of times to roll")
                .num_args(1)
                .default_value("1")
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("dice")
                .help("Dice (see readme for syntax)")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true),
        )
        .get_matches();

    // println!("{:#?}", m);
    if m.get_flag("macro") {
        println!("oops not implemented yet");
        process::exit(1);
    }

    let dice: &String = m.get_one("dice").expect("default");
    let mut chunks = parse_dice(dice, CONTROL_CHARS);
    chunks.reverse(); // so push/pop work as expected

    let mut rng = rand::thread_rng();

    while let Some(chunk) = chunks.pop() {
        println!("{}", chunk);
    }

    // loop {
    //     // will contain parser logic until refactored ?
    //     match chunks.pop() {
    //         Some(chunk) => {
    //             match chunk.pop() {
    //                 Some(char) => {
    //                     match char
    //                 }

    //             }
    //         }
    //         None => {
    //             break;
    //         }
    //     }
    // }
}

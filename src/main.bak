use clap::builder::{Arg, ArgGroup};
use clap::Command;
use rand::rngs::ThreadRng;
use rand::Rng;

// TODO: implement two-char control sequences
// for >=, <=, !=
//
// TODO: implement parens "()"
// TODO: implement brackets for macro names "[]"
// needs more design work -- how do
const CONTROL_CHARS: &[char] = &['d', '>', '<', '-', '+', '='];

#[derive(Debug, Clone, Copy)]
enum Chunk {
    Op(char),
    Num(u32),
}

impl TryFrom<char> for Chunk {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, &'static str> {
        match c {
            _ if CONTROL_CHARS.contains(&c) => Ok(Chunk::Op(c)),
            _ if c.is_ascii_digit() => Ok(Chunk::Num(c.to_digit(10).unwrap())),
            _ => Err("not a valid character"),
        }
    }
}

fn process_char(
    c: char,
    expression: &mut Vec<Chunk>,
    rng: &mut ThreadRng,
) -> Result<(), &'static str> {
    let curr_chunk = Chunk::try_from(c).expect("invalid character: {c}");
    let expression_top = expression.last().cloned();
    match (curr_chunk, expression_top) {
        (Chunk::Op(_), None) => Err("tried to add operator to empty expression"),
        (Chunk::Op(_), Some(Chunk::Op(_))) => Err("tried to push two operators to the expression"), // TODO: implement operator merging
        (Chunk::Num(_), Some(Chunk::Op(_))) | (Chunk::Num(_), None) => {
            expression.push(curr_chunk);
            Ok(())
        }
        (Chunk::Num(n1), Some(Chunk::Num(n10))) => {
            let _ = expression.pop();
            expression.push(Chunk::Num(n10 * 10 + n1));
            Ok(())
        }
        (Chunk::Op(_), Some(Chunk::Num(_))) => {
            let result = evaluate(expression, rng)?;
            expression.clear();
            expression.push(Chunk::Num(result));
            expression.push(curr_chunk);
            Ok(())
        }
    }
}

fn evaluate(expression: &[Chunk], rng: &mut ThreadRng) -> Result<u32, &'static str> {
    match expression {
        [Chunk::Num(n)] => Ok(*n),
        [Chunk::Num(n1), Chunk::Op(c), Chunk::Num(n2)] => {
            println!("{n1} {c} {n2}");
            match c {
                'd' => Ok((0..*n1).fold::<u32, _>(0, |s, _| s + ((rng.gen::<u32>() % n2) + 1))),
                '+' => Ok(n1 + n2),
                '-' => {
                    if n2 > n1 {
                        Ok(0)
                    } else {
                        Ok(n1 - n2)
                    }
                }
                '=' => Ok(if n1 == n2 { 1 } else { 0 }),
                '>' => Ok(if n1 > n2 { 1 } else { 0 }),
                '<' => Ok(if n1 < n2 { 1 } else { 0 }),
                _ => Err("unimplemented control char snuck through: {c}"),
            }
        }
        _ => Err("malformed expression: {expression}"),
    }
}

fn main() -> Result<(), &'static str> {
    let m = Command::new("rx")
        .about("Rolls dice.")
        .arg(
            Arg::new("dice")
                .short('d')
                .help("Sums rolled dice to one value. Check README for syntax.")
                .num_args(0),
        )
        .arg(
            Arg::new("macro")
                .short('m')
                .help("Rolls on a user-defined macro [NOT IMPLEMENTED YET]")
                .num_args(0),
        )
        .arg(
            Arg::new("table")
                .short('t')
                .help("Rolls on a user-defined table [NOT IMPLEMENTED YET]")
                .num_args(0),
        )
        .group(
            ArgGroup::new("mode")
                .id("mode")
                .args(["dice", "macro", "table"])
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
            Arg::new("args")
                .help("Arguments: either a dice-string or the name of a table/macro")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true),
        )
        .get_matches();

    if m.get_flag("macro") || m.get_flag("table") {
        todo!()
    }

    let dice: &String = m.get_one::<String>("args").expect("default");
    let feed = dice.chars();
    let mut rng = rand::thread_rng();
    let mut expression = Vec::<Chunk>::new();

    for i in 1..(*m.get_one::<u16>("rolls").unwrap() + 1) {
        for c in feed.clone() {
            match process_char(c, &mut expression, &mut rng) {
                Ok(()) => continue,
                Err(e) => eprintln!("{e}"),
            }
        }
        let result = evaluate(&expression, &mut rng)?;
        println!("result no.{i}: {result}");
        expression.clear();
    }
    Ok(())
}

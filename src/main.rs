use clap::Parser;
use rand::prelude::*;
use std::ops::RangeInclusive;

struct Password {
    length: u16,
}

impl Password {
    const CHAR_SET: RangeInclusive<char> = '\u{0021}'..='\u{007E}';

    fn generate(&self) -> String {
        let mut rng = StdRng::from_entropy();
        (0..self.length)
            .map(|_| Self::CHAR_SET.choose(&mut rng).unwrap())
            .collect()
    }
}

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    /// The length of the generated password.
    #[clap(short = 'n', long)]
    length: u16,
}

fn main() {
    let args = Args::parse();
    let passwd = Password {
        length: args.length,
    };
    let passwd_str = passwd.generate();
    println!("{}", passwd_str);
}

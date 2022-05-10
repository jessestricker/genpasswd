use clap::{ArgEnum, Parser};
use genpasswd::chars::{Partition, PartitionSet};
use genpasswd::Password;
use itertools::Itertools;
use log::{error, info};

/// An easy-peasy password generator for the command line.
///
/// The requested number of randomly generated password strings
/// are printed line-by-line to standard output.
///
/// By default, `genpasswd` does **not** enforce that at least one
/// character of each category (e.g. letter/digit/symbol) picked.
#[derive(Parser)]
#[clap(version, author, term_width = 80)]
struct Args {
    /// The length of each generated password.
    #[clap(short, long)]
    length: u16,

    /// The number of generated passwords.
    #[clap(short = 'n', long, default_value_t = 1)]
    count: u64,

    /// The set of characters to choose from.
    #[clap(short, long, arg_enum, default_value_t = Type::Ascii)]
    r#type: Type,

    /// Print information about the generated passwords to standard error.
    #[clap(short, long)]
    verbose: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}

#[derive(ArgEnum, Clone)]
enum Type {
    Letters,
    Digits,
    LettersDigits,
    Ascii,
}

impl Type {
    fn chars(&self) -> PartitionSet {
        use Partition::*;
        match self {
            Type::Letters => PartitionSet::from([LettersUpper, LettersLower]),
            Type::Digits => PartitionSet::from([Digits]),
            Type::LettersDigits => PartitionSet::from([LettersUpper, LettersLower, Digits]),
            Type::Ascii => PartitionSet::from([LettersUpper, LettersLower, Digits, Symbols]),
        }
    }
}

fn main() {
    let args = Args::parse();

    setup_logging(&args);
    validate_cli(&args);

    // set up password generator
    let passwd = Password::new(args.r#type.chars(), args.length);

    // log password generator properties
    let chars_str = passwd.chars().iter().join("");
    info!("Characters: {}", chars_str);
    info!("Length: {}", args.length);
    info!("Count: {}", args.count);

    // generate passwords
    for _ in 0..args.count {
        let passwd_str = passwd.generate();
        println!("{}", passwd_str);
    }
}

fn setup_logging(args: &Args) {
    use std::io::Write;

    let log_level = if args.verbose {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    };

    env_logger::Builder::new()
        .filter(None, log_level)
        .format(|fmt, record| {
            let level_style = fmt.default_level_style(record.level());
            let level_str = format!("{}:", record.level().as_str().to_ascii_lowercase());

            writeln!(fmt, "{} {}", level_style.value(level_str), record.args())
        })
        .init();
}

fn validate_cli(args: &Args) {
    if args.length == 0 {
        error!("The length must not be zero");
        std::process::exit(1);
    }

    if args.count == 0 {
        error!("The count must not be zero");
        std::process::exit(1);
    }
}

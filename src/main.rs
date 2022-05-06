use clap::{ArgEnum, Parser};
use genpasswd::{chars, CharSet, Password};
use lazy_static::lazy_static;

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    /// The length of the generated password.
    #[clap(short = 'n', long)]
    pub length: u16,

    /// The set of characters to choose from.
    #[clap(short, long, arg_enum, default_value_t = Type::Ascii)]
    pub r#type: Type,

    /// Write information about the generated password to `stderr`.
    #[clap(short, long)]
    verbose: bool,
}

#[derive(ArgEnum, Clone)]
enum Type {
    Letters,
    Digits,
    LettersDigits,
    Ascii,
}

lazy_static! {
    static ref LETTERS: CharSet = &*chars::LETTERS_UPPER | &*chars::LETTERS_LOWER;
    static ref LETTERS_AND_DIGITS: CharSet = &*LETTERS | &*chars::DIGITS;
    static ref ASCII: CharSet = &*LETTERS_AND_DIGITS | &*chars::SYMBOLS;
}

impl Type {
    fn char_set(&self) -> &'static CharSet {
        match self {
            Type::Letters => &LETTERS,
            Type::Digits => &chars::DIGITS,
            Type::LettersDigits => &LETTERS_AND_DIGITS,
            Type::Ascii => &ASCII,
        }
    }
}

fn main() {
    let args = Args::parse();

    let passwd = Password {
        char_set: args.r#type.char_set(),
        length: args.length,
    };

    if args.verbose {
        eprintln!("{}", passwd);
    }

    let passwd_str = passwd.generate();
    println!("{}", passwd_str);
}

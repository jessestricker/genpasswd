use std::collections::BTreeSet;

/// An ordered set of unique characters.
pub type CharSet = BTreeSet<char>;

pub mod chars {
    use super::CharSet;
    use lazy_static::lazy_static;
    use std::ops::RangeInclusive;

    const GRAPHIC: RangeInclusive<char> = '\x21'..='\x7E';

    lazy_static! {
        /// The set of uppercase letters in the _Basic Latin_ block.
        ///
        /// Generated from `[[:blk=ASCII:]&[:gc=Lu:]]`.
        pub static ref LETTERS_UPPER: CharSet = ('A'..='Z').collect();

        /// The set of lowercase letters in the _Basic Latin_ block.
        ///
        /// Generated from `[[:blk=ASCII:]&[:gc=Ll:]]`.
        pub static ref LETTERS_LOWER: CharSet = ('a'..='z').collect();

        /// The set of decimal digits in the _Basic Latin_ block.
        ///
        /// Generated from `[[:blk=ASCII:]&[:gc=Nd:]]`.
        pub static ref DIGITS: CharSet = ('0'..='9').collect();

        /// The set of symbols and punctuation in the _Basic Latin_ block.
        ///
        /// Generated from `[[:blk=ASCII:]&[[:gc=S:]|[:gc=P:]]]`.
        pub static ref SYMBOLS: CharSet = GRAPHIC.filter(char::is_ascii_punctuation).collect();
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn all_sets_are_disjoint() {
            assert!(LETTERS_UPPER.is_disjoint(&LETTERS_LOWER));
            assert!(LETTERS_UPPER.is_disjoint(&DIGITS));
            assert!(LETTERS_UPPER.is_disjoint(&SYMBOLS));

            assert!(LETTERS_LOWER.is_disjoint(&DIGITS));
            assert!(LETTERS_LOWER.is_disjoint(&SYMBOLS));

            assert!(DIGITS.is_disjoint(&SYMBOLS));
        }

        #[test]
        fn all_sets_unite_to_graphics() {
            let all_sets: CharSet = (LETTERS_UPPER.iter())
                .chain(LETTERS_LOWER.iter())
                .chain(DIGITS.iter())
                .chain(SYMBOLS.iter())
                .cloned()
                .collect();
            let graphic_set: CharSet = GRAPHIC.collect();
            assert_eq!(graphic_set, all_sets);
        }
    }
}

/// The properties from which a password string is generated.
pub struct Password<'a> {
    /// The set of characters to randomly choose from.
    pub char_set: &'a CharSet,

    /// The character length of the password.
    pub length: u16,
}

impl Password<'_> {
    /// Generates a random password string.
    pub fn generate(&self) -> String {
        use rand::prelude::*;

        let mut rng = StdRng::from_entropy();

        (0..self.length)
            .map(|_| self.char_set.iter().choose(&mut rng).unwrap())
            .collect()
    }
}

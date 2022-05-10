use std::collections::BTreeSet;

/// A partition of all *graphical* characters in the Unicode block _Basic Latin_.
///
/// The notation used below is documented in
/// [ICU Documentation, UnicodeSet](https://unicode-org.github.io/icu/userguide/strings/unicodeset.html).
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Partition {
    /// The uppercase letters.
    ///
    /// Defined as `[[:Block=Basic_Latin:] & [:General_Category=Uppercase_Letter:]]`.
    LettersUpper,

    /// The lowercase letters.
    ///
    /// Defined as `[[:Block=Basic_Latin:] & [:General_Category=Lowercase_Letter:]]`.
    LettersLower,

    /// The decimal digits.
    ///
    /// Defined as `[[:Block=Basic_Latin:] & [:General_Category=Decimal_Number:]]`.
    Digits,

    /// The punctuation marks and symbols.
    ///
    /// Defined as `[[:Block=Basic_Latin:] & [[:General_Category=Punctuation:] [:General_Category=Symbol:]]]`.
    Symbols,
}

impl Partition {
    const LETTERS_UPPER: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    const LETTERS_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    const SYMBOLS: [char; 32] = [
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ];

    /// Returns a slice of all characters in the partition.
    pub fn chars(&self) -> &'static [char] {
        match self {
            Partition::LettersUpper => &Partition::LETTERS_UPPER,
            Partition::LettersLower => &Partition::LETTERS_LOWER,
            Partition::Digits => &Partition::DIGITS,
            Partition::Symbols => &Partition::SYMBOLS,
        }
    }
}

impl IntoIterator for &Partition {
    type Item = &'static char;
    type IntoIter = std::slice::Iter<'static, char>;

    fn into_iter(self) -> Self::IntoIter {
        self.chars().iter()
    }
}

/// An ordered set of unique characters.
pub type CharSet = BTreeSet<char>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partitions_are_disjoint() {
        let letters_upper: CharSet = Partition::LettersUpper.into_iter().cloned().collect();
        let letters_lower: CharSet = Partition::LettersLower.into_iter().cloned().collect();
        let digits: CharSet = Partition::Digits.into_iter().cloned().collect();
        let symbols: CharSet = Partition::Symbols.into_iter().cloned().collect();

        assert!(letters_upper.is_disjoint(&letters_lower));
        assert!(letters_upper.is_disjoint(&digits));
        assert!(letters_upper.is_disjoint(&symbols));

        assert!(letters_lower.is_disjoint(&digits));
        assert!(letters_lower.is_disjoint(&symbols));

        assert!(digits.is_disjoint(&symbols));
    }

    #[test]
    fn partitions_unite_to_graphic() {
        let graphic: CharSet = ('\x00'..='\x7F').filter(|x| x.is_ascii_graphic()).collect();

        let union: CharSet = [
            Partition::LettersUpper,
            Partition::LettersLower,
            Partition::Digits,
            Partition::Symbols,
        ]
        .iter()
        .flatten()
        .cloned()
        .collect();

        assert_eq!(union, graphic);
    }
}

/// A union of character partitions.
pub type PartitionSet = BTreeSet<Partition>;

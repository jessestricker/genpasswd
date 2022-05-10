pub mod chars;
use chars::PartitionSet;

/// The properties from which a password string is generated.
pub struct Password {
    /// The characters to randomly choose from.
    chars: Vec<char>,

    /// The character length of the password.
    length: u16,
}

impl Password {
    /// Creates a new password generator.
    ///
    /// # Panics
    ///
    /// The following conditions must be true, or else this method will panic.
    ///
    /// * `parts` must not be empty
    /// * `length` must not be 0
    pub fn new(parts: PartitionSet, length: u16) -> Self {
        assert!(!parts.is_empty());
        assert!(length > 0);

        let chars = parts.iter().flatten().cloned().collect();

        Self { length, chars }
    }

    /// Returns the vector
    pub fn chars(&self) -> &Vec<char> {
        &self.chars
    }

    /// Generates a random password string.
    pub fn generate(&self) -> String {
        use rand::prelude::*;

        let mut rng = StdRng::from_entropy();

        (0..self.length)
            .map(|_| self.chars.choose(&mut rng).unwrap())
            .collect()
    }
}

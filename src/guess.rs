use crate::{constants::ALPHABET, constants::WORD_LENGTH};

pub enum LetterResult {
    GREEN,
    YELLOW,
    BLACK,
}

pub struct Guess {
    pub letters: [char; WORD_LENGTH],
    pub hints: [LetterResult; WORD_LENGTH],
}

// TODO: Implement an iterator so you can do
//       for (letter, hint) in Guess

impl Guess {
    pub fn new(word: &str, hints: [LetterResult; WORD_LENGTH]) -> Option<Guess> {
        if word.chars().count() != WORD_LENGTH {
            return None;
        }

        for letter in word.chars() {
            if !ALPHABET.contains(&letter) {
                return None;
            }
        }

        Some(Guess {
            letters: word
                .chars()
                .collect::<Vec<char>>()
                .try_into() // try to convert into [char; WORD_LENGTH]
                .expect(format!("Guess must contain exactly {} characters", WORD_LENGTH).as_str()),
            hints: hints,
        })
    }
}

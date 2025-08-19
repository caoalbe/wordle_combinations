use crate::{constants::ALPHABET, constants::WORD_LENGTH};

#[derive(Clone, Copy)]
pub enum LetterResult {
    GREEN,
    YELLOW,
    BLACK,
}

pub struct Guess {
    pub letters: [char; WORD_LENGTH],
    pub hints: [LetterResult; WORD_LENGTH],
}

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

    pub fn iter(&self) -> GuessIter<'_> {
        GuessIter {
            guess: self,
            index: 0,
        }
    }
}

pub struct GuessIter<'a> {
    guess: &'a Guess,
    index: usize,
}

impl<'a> Iterator for GuessIter<'a> {
    type Item = (char, LetterResult);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < WORD_LENGTH {
            let i = self.index;
            self.index += 1;
            Some((self.guess.letters[i], self.guess.hints[i]))
        } else {
            None
        }
    }
}

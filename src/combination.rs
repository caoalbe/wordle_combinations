use std::char;

use crate::{
    constants::{ALPHABET, WORD_LENGTH},
    guess::{Guess, LetterResult},
    superposition::Superposition,
};

#[derive(Clone)]
pub struct CombinationStore {
    pub possible_chars: [Superposition; WORD_LENGTH],
    pub must_contain: Vec<char>,
}

impl CombinationStore {
    pub fn new() -> CombinationStore {
        CombinationStore {
            possible_chars: std::array::from_fn::<Superposition, WORD_LENGTH, _>(|_| {
                Superposition::Unknown(ALPHABET.to_vec())
            }),
            must_contain: vec![],
        }
    }

    fn push_must_contain(&mut self, new_letter: char) {
        if !self.must_contain.contains(&new_letter) {
            self.must_contain.push(new_letter);
        }
    }

    pub fn add_guess(&mut self, guess: Guess) {
        for (c, (letter, hint)) in guess.iter().enumerate() {
            match hint {
                LetterResult::BLACK => {
                    for i in 0..WORD_LENGTH {
                        self.possible_chars[i].drop_state(letter);
                    }
                }
                LetterResult::GREEN => {
                    self.possible_chars[c] = Superposition::Known(letter);
                    self.push_must_contain(letter)
                }
                LetterResult::YELLOW => {
                    self.possible_chars[c].drop_state(letter);
                    self.push_must_contain(letter)
                }
            }
        }
    }

    pub fn print(&self) {
        for state in &self.possible_chars {
            match state {
                Superposition::Known(val) => {
                    print!("{}", val);
                }
                Superposition::Unknown(vec) => {
                    print!("(");
                    for val in vec {
                        print!("{val}");
                    }
                    print!(")");
                }
            }
        }
        print!("\n");
    }

    pub fn print_possible_combos(&self) {
        let patterns: Vec<CombinationStore> = self.generate_patterns();

        for pattern in patterns {
            CombinationStore::print_pattern_combos_helper(&pattern);
        }
    }

    fn generate_patterns(&self) -> Vec<CombinationStore> {
        // Generate an exhaustive list of patterns which are distinct,
        // and satisfy all possible placements of yellow letters
        let mut must_contain_vec = self.must_contain.clone();

        let mut patterns: Vec<CombinationStore> = vec![self.clone()];
        while let Some(letter) = must_contain_vec.pop() {
            // Exhaust every remaining combination
            patterns = patterns
                .iter()
                .cloned()
                .flat_map(|p: CombinationStore| {
                    let mut partial_pattern: Vec<CombinationStore> = vec![p.clone()];
                    let mut letter_is_green: bool = false;
                    for (i, place) in p.possible_chars.iter().enumerate() {
                        match place {
                            Superposition::Known(to_compare) => {
                                if to_compare == &letter {
                                    letter_is_green = true;
                                }
                            }
                            Superposition::Unknown(vec) => {
                                if vec.contains(&letter) {
                                    partial_pattern = partial_pattern
                                        .iter()
                                        .flat_map(|state| {
                                            let mut collapsed = state.clone();
                                            let mut ambiguous = state.clone();

                                            collapsed.possible_chars[i] =
                                                Superposition::Known(letter);
                                            ambiguous.possible_chars[i].drop_state(letter);

                                            vec![collapsed, ambiguous]
                                        })
                                        .collect();
                                }
                            }
                        }
                    }

                    if !letter_is_green {
                        // Note that the last pattern is one where
                        // <letter> is selected in none of the unknown positions

                        // Naturally, this pattern is valid iff
                        // <letter> is already in a known position
                        partial_pattern.pop();
                    }

                    partial_pattern
                })
                .collect();
        }
        patterns
    }

    fn print_pattern_combos_helper(pattern: &CombinationStore) {
        // Exhaust every remaining combination
        let mut i = 0;
        while i < WORD_LENGTH && matches!(pattern.possible_chars[i], Superposition::Known(_)) {
            i += 1;
        }

        if i == WORD_LENGTH {
            pattern.print();
        } else {
            // collapse at index i
            if let Superposition::Unknown(vec) = &pattern.possible_chars[i] {
                for letter in vec {
                    let mut dummy = pattern.clone();
                    dummy.possible_chars[i] = Superposition::Known(*letter);
                    CombinationStore::print_pattern_combos_helper(&dummy);
                }
            }
        }
    }
}

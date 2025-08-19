use crate::{
    constants::{ALPHABET, WORD_LENGTH},
    guess::{Guess, LetterResult},
    superposition::{Superposition, superposition_drop_state},
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

    // TODO: Handle double letters (GREEN & BLACK) in one guess
    pub fn add_guess(&mut self, guess: Guess) {
        for c in 0..WORD_LENGTH {
            match guess.hints[c] {
                LetterResult::BLACK => {
                    for i in 0..WORD_LENGTH {
                        superposition_drop_state(guess.letters[c], &mut self.possible_chars[i]);
                    }
                }
                LetterResult::GREEN => {
                    self.possible_chars[c] = Superposition::Known(guess.letters[c]);
                    self.must_contain.push(guess.letters[c])
                }
                LetterResult::YELLOW => {
                    superposition_drop_state(guess.letters[c], &mut self.possible_chars[c]);
                    self.must_contain.push(guess.letters[c])
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
}

pub fn print_helper(combo: CombinationStore, mut must_include: Vec<char>) {
    // check if every superposition has collapsed and must_include is empty

    match must_include.pop() {
        None => {
            // Exhaust every remaining combination
            let mut i = 0;
            while i < WORD_LENGTH && matches!(&combo.possible_chars[i], Superposition::Known(_)) {
                i += 1;
            }

            if i == WORD_LENGTH {
                combo.print();
            } else {
                // collapse at index i
                if let Superposition::Unknown(vec) = &combo.possible_chars[i] {
                    for letter in vec {
                        let mut dummy = combo.clone();
                        dummy.possible_chars[i] = Superposition::Known(*letter);
                        print_helper(dummy, vec![]);
                    }
                }
            }
        }
        Some(letter) => {
            // Find all satisfying combinations for this letter
            let mut letter_combos: Vec<CombinationStore> = vec![combo.clone()];
            let mut i = 0;

            let mut letter_already_placed = false;
            for place in &combo.possible_chars {
                match place {
                    Superposition::Known(to_compare) => {
                        if to_compare == &letter {
                            letter_already_placed = true;
                        }
                    }
                    Superposition::Unknown(vec) => {
                        if vec.contains(&letter) {
                            letter_combos = letter_combos
                                .iter()
                                .flat_map(|state| {
                                    let mut collapsed = state.clone();
                                    let mut ambiguous = state.clone();

                                    collapsed.possible_chars[i] = Superposition::Known(letter);
                                    superposition_drop_state(
                                        letter,
                                        &mut ambiguous.possible_chars[i],
                                    );

                                    vec![collapsed, ambiguous]
                                })
                                .collect();
                        }
                    }
                }
                i = i + 1;
            }

            if !letter_already_placed {
                letter_combos.pop(); // the last one is invalid; since it places the must_include letter nowhere
            }

            for combo in letter_combos {
                print_helper(combo, must_include.clone());
            }
        }
    }
}

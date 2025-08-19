use crate::{
    constants::{ALPHABET, WORD_LENGTH},
    guess::{Guess, LetterResult},
    superposition::{Superposition},
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
        for c in 0..WORD_LENGTH {
            match guess.hints[c] {
                LetterResult::BLACK => {
                    for i in 0..WORD_LENGTH {
                        self.possible_chars[i].drop_state(guess.letters[c]);
                    }
                }
                LetterResult::GREEN => {
                    self.possible_chars[c] = Superposition::Known(guess.letters[c]);
                    self.push_must_contain(guess.letters[c])
                }
                LetterResult::YELLOW => {
                    self.possible_chars[c].drop_state(guess.letters[c]);
                    self.push_must_contain(guess.letters[c])
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
        let must_contain_vec = self.must_contain.clone();
        self.print_possible_combos_helper(must_contain_vec);
    }

    fn print_possible_combos_helper(&self, mut must_include: Vec<char>) {
        match must_include.pop() {
            None => {
                // Exhaust every remaining combination
                let mut i = 0;
                while i < WORD_LENGTH && matches!(self.possible_chars[i], Superposition::Known(_)) {
                    i += 1;
                }

                if i == WORD_LENGTH {
                    self.print();
                } else {
                    // collapse at index i
                    if let Superposition::Unknown(vec) = &self.possible_chars[i] {
                        for letter in vec {
                            let mut dummy = self.clone();
                            dummy.possible_chars[i] = Superposition::Known(*letter);
                            dummy.print_possible_combos_helper(vec![]);
                        }
                    }
                }
            }
            Some(letter) => {
                // Find all satisfying combinations for this letter
                let mut letter_combos: Vec<CombinationStore> = vec![self.clone()];
                let mut i = 0;

                let mut letter_already_placed = false;
                for place in &self.possible_chars {
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
                                        ambiguous.possible_chars[i].drop_state(letter);

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
                    combo.print_possible_combos_helper(must_include.clone());
                }
            }
        }
    }
}

use std::char;

const WORD_LENGTH: usize = 5;
const ALPHABET: [char; 26] = [
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
    'z', 'x', 'c', 'v', 'b', 'n', 'm',
];
// const ALPHABET: [char; 22] = [
//     'w', 'e', 'r', 't', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'c', 'v',
//     'b', 'n', 'm',
// ];

enum LetterResult {
    GREEN,
    YELLOW,
    BLACK,
}

struct Guess {
    letters: [char; WORD_LENGTH],
    hints: [LetterResult; WORD_LENGTH],
}

// TODO: Implement an iterator so you can do
//       for (letter, hint) in Guess

impl Guess {
    fn new(word: &str, hints: [LetterResult; WORD_LENGTH]) -> Option<Guess> {
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

#[derive(Clone)]
enum Superposition {
    Known(char),
    Unknown(Vec<char>),
}

fn superposition_drop_state(c: char, target: &mut Superposition) {
    if let Superposition::Unknown(vec) = target {
        vec.retain(|&x| x != c);
        if vec.len() == 1 {
            let last = vec.pop().unwrap();
            *target = Superposition::Known(last);
        }
    }
}

#[derive(Clone)]
struct CombinationStore {
    possible_chars: [Superposition; WORD_LENGTH],
    must_contain: Vec<char>,
}

impl CombinationStore {
    fn new() -> CombinationStore {
        CombinationStore {
            possible_chars: std::array::from_fn::<Superposition, WORD_LENGTH, _>(|_| {
                Superposition::Unknown(ALPHABET.to_vec())
            }),
            must_contain: vec![],
        }
    }

    // TODO: Handle double letters (GREEN & BLACK) in one guess
    fn add_guess(&mut self, guess: Guess) {
        for c in 0..WORD_LENGTH {
            match guess.hints[c] {
                LetterResult::BLACK => {
                    for i in 0..WORD_LENGTH {
                        superposition_drop_state(guess.letters[c], &mut self.possible_chars[i]);
                    }
                }
                LetterResult::GREEN => {
                    self.possible_chars[c] = Superposition::Known(guess.letters[c]);
                }
                LetterResult::YELLOW => {
                    superposition_drop_state(guess.letters[c], &mut self.possible_chars[c]);
                    self.must_contain.push(guess.letters[c])
                }
            }
        }
    }

    fn print(&self) {
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

fn print_helper(combo: CombinationStore, mut must_include: Vec<char>) {
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
            let mut letter_combos = vec![combo.clone()];

            let mut i = 0;
            for place in &combo.possible_chars {
                if let Superposition::Unknown(vec) = place {
                    if vec.contains(&letter) {
                        letter_combos = letter_combos
                            .iter()
                            .flat_map(|state| {
                                let mut collapsed = state.clone();
                                let mut ambiguous = state.clone();

                                collapsed.possible_chars[i] = Superposition::Known(letter);
                                superposition_drop_state(letter, &mut ambiguous.possible_chars[i]);

                                vec![collapsed, ambiguous]
                            })
                            .collect();
                    }
                }
                i = i + 1;
            }

            letter_combos.pop(); // the last one is invalid; since it places the must_include letter nowhere

            for combo in letter_combos {
                print_helper(combo, must_include.clone());
            }
        }
    }
}

fn main() {
    // wordle #1519. matte
    let mut solver: CombinationStore = CombinationStore::new();
    solver.add_guess(
        Guess::new(
            "ocean",
            [
                LetterResult::BLACK,
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::YELLOW,
                LetterResult::BLACK,
            ],
        )
        .unwrap(),
    );
    solver.add_guess(
        Guess::new(
            "steak",
            [
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::YELLOW,
                LetterResult::YELLOW,
                LetterResult::BLACK,
            ],
        )
        .unwrap(),
    );
    solver.add_guess(
        Guess::new(
            "heard",
            [
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::YELLOW,
                LetterResult::BLACK,
                LetterResult::BLACK,
            ],
        )
        .unwrap(),
    );
    solver.add_guess(
        Guess::new(
            "valet",
            [
                LetterResult::BLACK,
                LetterResult::GREEN,
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::YELLOW,
            ],
        )
        .unwrap(),
    );
    solver.add_guess(
        Guess::new(
            "table",
            [
                LetterResult::YELLOW,
                LetterResult::GREEN,
                LetterResult::BLACK,
                LetterResult::BLACK,
                LetterResult::GREEN,
            ],
        )
        .unwrap(),
    );

    // // wordle #1516. kefir
    // solver.add_guess(
    //     Guess::new(
    //         "ocean",
    //         [
    //             LetterResult::BLACK,
    //             LetterResult::BLACK,
    //             LetterResult::YELLOW,
    //             LetterResult::BLACK,
    //             LetterResult::BLACK,
    //         ],
    //     )
    //     .unwrap(),
    // );
    // solver.add_guess(
    //     Guess::new(
    //         "whine",
    //         [
    //             LetterResult::BLACK,
    //             LetterResult::BLACK,
    //             LetterResult::YELLOW,
    //             LetterResult::BLACK,
    //             LetterResult::YELLOW,
    //         ],
    //     )
    //     .unwrap(),
    // );
    // solver.add_guess(
    //     Guess::new(
    //         "diner",
    //         [
    //             LetterResult::BLACK,
    //             LetterResult::YELLOW,
    //             LetterResult::BLACK,
    //             LetterResult::YELLOW,
    //             LetterResult::GREEN,
    //         ],
    //     )
    //     .unwrap(),
    // );
    // solver.add_guess(
    //     Guess::new(
    //         "stump",
    //         [
    //             LetterResult::BLACK,
    //             LetterResult::BLACK,
    //             LetterResult::BLACK,
    //             LetterResult::BLACK,
    //             LetterResult::BLACK,
    //         ],
    //     )
    //     .unwrap(),
    // );

    // solver.print();
    // print_helper(solver, vec!['i', 'e']);
    print_helper(solver, vec!['t']);
    // solver.debug_decisions();
}

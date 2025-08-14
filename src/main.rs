use std::char;

// draft 1 = 537824
// draft 2 = 38416
const WORD_LENGTH: usize = 5;
const ALPHABET: [char; 26] = [
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
    'z', 'x', 'c', 'v', 'b', 'n', 'm',
];

enum LetterResult {
    GREEN,
    YELLOW,
    BLACK,
}

struct Guess {
    letters: [char; WORD_LENGTH],
    hints: [LetterResult; WORD_LENGTH],
}

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
                .collect::<Vec<char>>() // collect into Vec<char>
                .try_into() // try to convert into [char; WORD_LENGTH]
                .expect("String must be exactly 5 characters"),
            hints: hints,
        })
    }
}

struct Feasible {
    // guesses: Vec<Guess>,
    possible_chars: [Vec<char>; WORD_LENGTH],
    blacks: Vec<char>,
    yellows: Vec<char>,
    greens: Vec<char>,
    whites: Vec<char>,
    unsolved_count: usize, // how many letters remain to be solved
}

impl Feasible {
    fn new() -> Feasible {
        Feasible {
            possible_chars: [
                ALPHABET.to_vec(),
                ALPHABET.to_vec(),
                ALPHABET.to_vec(),
                ALPHABET.to_vec(),
                ALPHABET.to_vec(),
            ],
            // possible_chars: [ALPHABET.to_vec().clone(); WORD_LENGTH],
            blacks: vec![],
            yellows: vec![],
            greens: vec![],
            whites: vec![],
            unsolved_count: WORD_LENGTH,
        }
    }

    fn add_guess(&mut self, guess: Guess) {
        for c in 0..WORD_LENGTH {
            match guess.hints[c] {
                LetterResult::BLACK => {
                    for c2 in 0..WORD_LENGTH {
                        self.possible_chars[c2].retain(|&i| i != guess.letters[c]);
                    }
                }
                LetterResult::GREEN => {
                    self.possible_chars[c] = vec![guess.letters[c].clone()];
                }
                LetterResult::YELLOW => {
                    self.possible_chars[c].retain(|&i| i != guess.letters[c]);
                    self.yellows.push(guess.letters[c])
                }
            }
        }
    }

    fn print(&self) {
        let mut result: String;
        let mut count: usize = 0;

        for char0 in self.possible_chars[0].clone() {
            for char1 in self.possible_chars[1].clone() {
                for char2 in self.possible_chars[2].clone() {
                    for char3 in self.possible_chars[3].clone() {
                        for char4 in self.possible_chars[4].clone() {
                            result = format!("{}{}{}{}{}", char0, char1, char2, char3, char4);
                            count = count + 1;
                            println!("{}", result);
                        }
                    }
                }
            }
        }

        println!("combination count = {}", count);
    }

    fn estimate_combinations(&self) -> usize {
        11_881_376 // TODO: Use possible_chars to estimate how many combinations might remain
    }
}

fn main() {
    // wordle #1516. kefir
    let mut solver: Feasible = Feasible::new();
    solver.add_guess(
        Guess::new(
            "ocean",
            [
                LetterResult::BLACK,
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::BLACK,
                LetterResult::BLACK,
            ],
        )
        .unwrap(),
    );
    solver.add_guess(
        Guess::new(
            "whine",
            [
                LetterResult::BLACK,
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::BLACK,
                LetterResult::YELLOW,
            ],
        )
        .unwrap(),
    );
    solver.add_guess(
        Guess::new(
            "diner",
            [
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::BLACK,
                LetterResult::YELLOW,
                LetterResult::GREEN,
            ],
        )
        .unwrap(),
    );
    solver.add_guess(
        Guess::new(
            "stump",
            [
                LetterResult::BLACK,
                LetterResult::BLACK,
                LetterResult::BLACK,
                LetterResult::BLACK,
                LetterResult::BLACK,
            ],
        )
        .unwrap(),
    );

    // draft 1 = 537824
    // draft 2 = 38416
}

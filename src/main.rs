use std::{env, fs};
use wordle_combinations::{
    combination::CombinationStore,
    constants::WORD_LENGTH,
    guess::{Guess, LetterResult},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Insufficient arguments".into());
    }
    let input_file_path: &String = &args[1];

    let hints_string = fs::read_to_string(input_file_path)
        .expect(format!("Error reading {}.", input_file_path).as_str());

    // Parse hints_string into a CombinationStore
    let mut solver: CombinationStore = CombinationStore::new();
    let hints_string: Vec<&str> = hints_string.split("\n").collect();

    let mut reading_text: bool = true; // true -> reading text, false -> reading colour

    let mut word: &str = "";
    for mut line in hints_string {
        line = line.trim_start();
        if line.starts_with("//") || line == "" {
            continue;
        }

        if reading_text {
            word = line;
        } else {
            let hints: [LetterResult; WORD_LENGTH] = line
                .chars()
                .map(|colour| match colour {
                    '.' => LetterResult::BLACK,
                    '?' => LetterResult::YELLOW,
                    '!' => LetterResult::GREEN,
                    _ => {
                        println!("invalid character");
                        LetterResult::BLACK
                    }
                })
                .collect::<Vec<LetterResult>>()
                .try_into()
                .unwrap_or_else(|_| panic!("Must include {} hints.", WORD_LENGTH));

            solver.add_guess(Guess::new(word, hints).unwrap());
        }
        reading_text = !reading_text;
    }

    solver.print_possible_combos();
    Ok(())
}

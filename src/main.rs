use wordle_combinations::{combination::{CombinationStore, print_helper}, guess::{Guess, LetterResult}};

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

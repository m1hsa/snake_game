//! # snake_game
//! this is library for game binary, this library do many things for this game

use std::io::{self, Read};

/// hardcoded width of the game field, player can go maximum of W-1 positions
pub const W: u8 = 20;
/// hardcoded height of the game field, player can go maximum of H-1 positions
pub const H: u8 = 20;

/// struct for implementation of LCG algoritm
pub struct Rng {
    seed: u64,
}
impl Rng {
    /// Generates new `Rng` struct
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    /// Returns random u8 number using linear congruential generator
    pub fn rand(&mut self) -> u8 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.seed % W as u64) as u8 + 1
    }
}

/// # Draws main tui screen
/// - as arguments it needs positions of various things
/// ## draws something like this:
///
/// ```text
/// score: 13
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . o o o o . . . . . . . . . . .
/// . . . . o . . o . . . . . . . . . . .
/// . . . . o . . o . . . . . . . . . . .
/// . . . . o . . o o . . . . . . . . . .
/// . . . . o . . . . . . . . . . . . . .
/// . . . . o o . . . . . . . . . . . . .
/// . . . . . # . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . F . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// . . . . . . . . . . . . . . . . . . .
/// ```
pub fn show_pg(head: &(u8, u8), body: &Vec<(u8, u8)>, food: &(u8, u8), score: &u16) {
    let mut pg = format!("score: {}\n", score);
    for y in 0..W - 1 {
        for x in 0..H - 1 {
            let xy = (x + 1, y + 1);
            if head == &xy {
                pg.push_str("# ");
                continue;
            }
            if food == &xy {
                pg.push_str("F ");
                continue;
            }
            if body.contains(&xy) {
                pg.push_str("o ");
                continue;
            }
            pg.push_str(". ");
        }
        pg.push('\n');
    }
    print!("{}\x1b[{}A", pg, H);
}

/// checks position of `point`
/// ```
/// use snake_game::*;
///
/// let mut point = (0, 10); /// x = 0 is out of bounds
/// check_borders(&mut point);
/// assert_eq!(point, (W-1,10));
///
/// let mut point2 = (10, H); /// y = H is out of bounds
/// check_borders(&mut point2);
/// assert_eq!(point2, (10,1));
/// ```
pub fn check_borders(point: &mut (u8, u8)) {
    *point = match *point {
        (0, x) => (W - 1, x),
        (x, 0) => (x, H - 1),
        (W, x) => (1, x),
        (x, H) => (x, 1),
        (x, y) => (x, y),
    };
}

/// handles keyboard.
/// BUG, if pressed something else than wasdhjkl aborts game
pub fn handle_keyboard(stdin: &mut io::Stdin, quit: &mut bool, head: &mut (u8, u8)) {
    let mut buf = [0];
    let _ = stdin.read_exact(&mut buf);

    match buf[0] {
        119 => head.1 -= 1, // 'w'
        97 => head.0 -= 1,  // 'a'
        115 => head.1 += 1, // 's'
        100 => head.0 += 1, // 'd'
        // vim motions
        104 => head.0 -= 1, // 'h'
        106 => head.1 += 1, // 'j'
        107 => head.1 -= 1, // 'k'
        108 => head.0 += 1, // 'l'
        // quit
        113 => *quit = true, // 'q'
        _ => (),
    }
}

/// checks if head is in the any part the body
/// if so aborts game
pub fn check_game_over(body: &Vec<(u8, u8)>, head: &(u8, u8), quit: &mut bool) {
    for i in body {
        if head == i {
            *quit = true;
        }
    }
}

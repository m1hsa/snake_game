use std::io::{self, Read};

pub const W: u8 = 20;
pub const H: u8 = 20;

pub struct Rng {
    seed: u64,
}
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    pub fn rand(&mut self) -> u8 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.seed % W as u64) as u8 + 1
    }
}

pub fn show_pg(head: &(u8, u8), body: &Vec<(u8, u8)>, food: &(u8, u8), score: &u16) {
    let mut pg = format!("score: {}\n", score);
    for y in 0..W - 1 {
        'x: for x in 0..H - 1 {
            if head.0 == x + 1 && head.1 == y + 1 {
                pg.push_str("# ");
                continue;
            }
            if food.0 == x + 1 && food.1 == y + 1 {
                pg.push_str("F ");
                continue;
            }
            for i in body {
                if i.0 == x + 1 && i.1 == y + 1 {
                    pg.push_str("o ");
                    continue 'x;
                }
            }
            pg.push_str(". ");
        }
        pg.push('\n');
    }
    print!("{}\x1b[{}A", pg, H);
}

pub fn check_borders(point: &mut (u8, u8)) {
    *point = match *point {
        (0, x) => (W - 1, x),
        (x, 0) => (x, H - 1),
        (W, x) => (1, x),
        (x, H) => (x, 1),
        (x, y) => (x, y),
    };
}

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

pub fn check_game_over(body: &Vec<(u8, u8)>, head: &(u8, u8), quit: &mut bool) {
    for i in body {
        if head == i {
            *quit = true;
        }
    }
}

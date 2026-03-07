mod c;
use std::io::{self, Read};

const W: u8 = 20;
const H: u8 = 20;

struct Rng {
    seed: u64,
}
impl Rng {
    fn rand(&mut self) -> u8 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.seed % W as u64) as u8 +1
    }
}

fn wait_for_key(mut stdin: &io::Stdin) -> u8 {
    let mut buf = [0];
    let _ = stdin.read_exact(&mut buf);
    buf[0]
}

fn translate_key(key: u8) -> char {
    match key {
        119 => 'w',
        97 => 'a',
        115 => 's',
        100 => 'd',
        113 => 'q',
        _ => 'z',
    }
}

fn show_pg(player_head: &(u8, u8), food: &(u8, u8), score: &u16) {
    // let mut pg = String::from(format!("{:?}\n",player_head));
    let mut pg = format!("{}\n", score);
    for y in 0..W-1 {
        for x in 0..H-1 {
            if player_head.0 == x + 1 && player_head.1 == y + 1 {
                pg.push_str("I ");
                continue;
            }
            if food.0 == x + 1 && food.1 == y + 1 {
                pg.push_str("F ");
                continue;
            }
            pg.push_str(". ");
        }
        pg.push('\n');
    }
    print!("{}\x1b[{}A", pg, H);
}

fn check_borders(point: &mut (u8, u8)) {
    *point = match *point {
        (0, x) => (W - 1, x),
        (x, 0) => (x, H - 1),
        (W, x) => (1, x),
        (x, H) => (x, 1),
        (x, y) => (x, y),
    };
}

fn main() {
    let term = c::set_term();
    let stdin = io::stdin();

    let mut player_head = (W / 2, H / 2);
    let mut r = Rng { seed: 42 };
    let mut score: u16 = 0;

    let mut food = (r.rand(), r.rand());
    check_borders(&mut food);

    loop {
        match translate_key(wait_for_key(&stdin)) {
            'w' => player_head.1 -= 1,
            'a' => player_head.0 -= 1,
            's' => player_head.1 += 1,
            'd' => player_head.0 += 1,
            'q' => break,
            _ => (),
        }

        check_borders(&mut player_head);

        if player_head == food {
            score += 1;
            food = (r.rand(),r.rand());
            check_borders(&mut food);
        }

        show_pg(&player_head, &food, &score);
    }

    c::remove_term(term);
}

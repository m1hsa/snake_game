use snake_game::*;
use std::io;
mod c;

fn main() {
    let term = c::set_term();
    let mut stdin = io::stdin();

    let mut r = Rng::new(42);
    let mut score: u16 = 0;
    let mut quit = false;

    let mut head = (W / 2, H / 2);
    let mut food = (r.rand(), r.rand());

    check_borders(&mut food);

    while !quit {
        handle_keyboard(&mut stdin, &mut quit, &mut head);

        check_borders(&mut head);

        if head == food {
            score += 1;
            food = (r.rand(), r.rand());
            check_borders(&mut food);
        }

        show_pg(&head, &food, &score);
    }

    c::remove_term(term);
}

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

    let mut body: Vec<(u8, u8)> = vec![(0, 0)];

    check_borders(&mut food);

    while !quit {
        body.push(head);
        body.remove(0);

        handle_keyboard(&mut stdin, &mut quit, &mut head);

        check_borders(&mut head);

        check_game_over(&body, &head, &mut quit);

        if head == food {
            score += 1;
            body.insert(0, head);
            loop {
                food = (r.rand(), r.rand());
                check_borders(&mut food);
                if !body.contains(&food) {
                    break;
                }
            }
        }

        show_pg(&head, &body, &food, &score);
    }

    println!("Game over! Score: {}", score);

    c::remove_term(term);
}

use std::{thread::sleep, time::Duration};

use pancurses::{
    cbreak, curs_set, endwin, has_colors, init_pair, initscr, newwin, noecho, start_color, Input,
    Window, ACS_LARROW, ACS_RARROW, ACS_UARROW, A_ALTCHARSET, A_BOLD, COLOR_BLACK, COLOR_BLUE,
    COLOR_GREEN, COLOR_PAIR, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};

use crate::{
    objects::{ObjectField, Rect},
    player::Player,
};

pub fn init() -> (Window, Window, Rect, Rect) {
    let mut main_wnd = initscr();
    cbreak();
    noecho();
    main_wnd.clear();
    main_wnd.refresh();

    curs_set(0);

    start_color();

    let screen_area = Rect {
        offset: (0, 0),
        bounds: (80, 24),
    };

    let infopanel_height = 4;
    let game_wnd = newwin(
        screen_area.height() - infopanel_height - 2,
        screen_area.width() - 2,
        screen_area.top() + 1,
        screen_area.left() + 1,
    );
    main_wnd = newwin(screen_area.height(), screen_area.width(), 0, 0);
    let game_area = Rect {
        offset: (0, 0),
        bounds: (
            screen_area.width() - 2,
            screen_area.height() - infopanel_height - 4,
        ),
    };

    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    main_wnd.bkgd(COLOR_PAIR(1));
    game_wnd.bkgd(COLOR_PAIR(1));
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_YELLOW, COLOR_BLACK);
    init_pair(4, COLOR_RED, COLOR_BLACK);
    init_pair(5, COLOR_BLUE, COLOR_BLACK);

    main_wnd.keypad(true);
    game_wnd.keypad(true);

    main_wnd.nodelay(true);
    game_wnd.nodelay(true);

    if !has_colors() {
        endwin();
        panic!("ERROR: Terminal does not support color.");
    }

    (main_wnd, game_wnd, game_area, screen_area)
}

pub fn run(main_wnd: Window, game_wnd: Window, game_area: Rect, screen_area: Rect) {
    let mut tick: i32;
    let mut player = Player::default();
    let mut stars: ObjectField = ObjectField::new(game_area, vec![]);
    let mut asteroids = ObjectField::new(game_area, vec![]);
    let mut game_over = false;

    // Frame
    main_wnd.attron(A_BOLD);
    main_wnd.draw_box(0, 0);
    main_wnd.attroff(A_BOLD);

    // Horizontal Divide
    main_wnd.mv(game_area.bottom() + 3, 1);
    main_wnd.hline('-', screen_area.width() - 2);

    // Initial Draw
    main_wnd.refresh();
    game_wnd.refresh();

    let story_text = [
        "Just another Monday, and you're on your way to work...",
        "When suddenly...",
        "You realise you left the oven on!",
        "Take a shortcut through the asteroid field!",
        "Get back to the house before your planet explodes!",
    ];

    main_wnd.mvprintw(22, 57, "press SPACE to skip...");

    tick = 0;
    let mut story_part = 0;
    let mut story_position = 0;
    loop {
        game_wnd.erase();
        if tick % 50 == 0 {
            stars.update();
        }
        stars.get_data().iter().for_each(|s| {
            game_wnd.mvaddch(s.get_pos().1, s.get_pos().0, '.');
        });
        if story_position < story_text[story_part].len() {
            main_wnd.attron(A_BOLD);
            main_wnd.mvaddch(
                20,
                5 + story_position as i32,
                story_text[story_part].chars().nth(story_position).unwrap(),
            );
            main_wnd.attroff(A_BOLD);
            story_position += 1;
        }

        if let Some(c) = main_wnd.getch() {
            match c {
                Input::Character(' ') => {
                    story_part += 1;
                    story_position = 0;
                    main_wnd.mv(20, 1);
                    main_wnd.hline(' ', screen_area.width() - 2);
                }
                Input::Character('q') => {
                    return;
                }
                _ => (),
            }
        }
        if story_part >= story_text.len() {
            break;
        }
        game_wnd.refresh();
        tick += 1;
        sleep(Duration::from_millis(1));
    }
    main_wnd.mv(22, 57);
    main_wnd.hline(' ', 22);

    tick = 0;
    loop {
        game_wnd.erase();

        if let Some(c) = main_wnd.getch() {
            if c == Input::Character('q') {
                break;
            } else {
                player.parse_input(c, &game_area);
            }
        }

        if tick % 7 == 0 {
            stars.update();
        }

        if tick > 100 && tick % 20 == 0 {
            asteroids.update();
        }

        player.bounds = Rect {
            offset: (player.pos.0 - 1, player.pos.1),
            bounds: (3, 2),
        };

        asteroids.object_set.retain(|asteroid| {
            if player.bounds.contains(asteroid.get_pos()) {
                player.energy -= 10;
                false
            } else {
                true
            }
        });

        if player.energy <= 0 {
            game_over = true;
        }

        stars.get_data().iter_mut().for_each(|star| {
            game_wnd.mvaddch(star.get_pos().1, star.get_pos().0, '.');
        });

        // Ship body
        game_wnd.attron(A_BOLD);
        game_wnd.mvaddch(player.pos.1, player.pos.0, player.disp_char);
        game_wnd.attroff(A_BOLD);

        game_wnd.attron(A_ALTCHARSET);
        game_wnd.mvaddch(player.pos.1, player.pos.0 - 1, ACS_LARROW());
        game_wnd.mvaddch(player.pos.1, player.pos.0 + 1, ACS_RARROW());

        if (tick / 5) % 3 == 0 {
            let color = if tick % 2 == 0 { 3 } else { 4 };
            game_wnd.attron(COLOR_PAIR(color));
            game_wnd.mvaddch(player.pos.1 + 1, player.pos.0, ACS_UARROW());
            game_wnd.attroff(COLOR_PAIR(color));
        }

        game_wnd.attroff(A_ALTCHARSET);

        game_wnd.attron(A_BOLD);
        asteroids.get_data().iter_mut().for_each(|asteroid| {
            game_wnd.mvaddch(asteroid.get_pos().1, asteroid.get_pos().0, '*');
        });
        game_wnd.attroff(A_BOLD);

        main_wnd.mv(20, 1);
        main_wnd.hline(' ', 25);
        main_wnd.mv(20, 1);
        draw_energy_bar(player.energy, &main_wnd);

        main_wnd.mvprintw(21, 1, " - E N E R G Y -      //");

        main_wnd.attron(A_BOLD);
        if player.energy <= 25 {
            main_wnd.attron(COLOR_PAIR(4));
            if tick % 100 < 50 {
                main_wnd.mvprintw(21, 18, format!("{}%", player.energy));
            }
            main_wnd.attroff(COLOR_PAIR(4));
        } else {
            main_wnd.mvprintw(21, 18, format!("{}", player.energy));
        }
        main_wnd.attroff(A_BOLD);

        main_wnd.refresh();
        game_wnd.refresh();

        if game_over {
            let xpos = game_area.width() / 2 - 6;
            let ypos = game_area.height() / 2 - 2;

            main_wnd.erase();
            main_wnd.attron(A_BOLD);
            main_wnd.draw_box(0, 0);
            main_wnd.attroff(A_BOLD);

            main_wnd.mv(game_area.bottom() + 3, 1);
            main_wnd.hline('-', screen_area.width() - 2);

            main_wnd.refresh();
            game_wnd.refresh();

            game_wnd.mvprintw(ypos, xpos, "GAME OVER");
            game_wnd.mvprintw(ypos + 2, xpos - 7, "Press SPACE to play again");
            game_wnd.mvprintw(ypos + 4, xpos - 7, "Press 'q' to quit the game");

            loop {
                if let Some(c) = main_wnd.getch() {
                    if c == Input::Character(' ') {
                        tick = 0;
                        player.pos = (10, 10);
                        player.energy = 100;
                        stars.get_data().clear();
                        asteroids.get_data().clear();
                        game_over = false;
                        break;
                    } else if c == Input::Character('q') {
                        return;
                    }
                    game_wnd.refresh();

                    tick += 1;
                    sleep(Duration::from_millis(1));
                }
            }
        }

        tick += 1;

        sleep(Duration::from_millis(10));
    }
}

pub fn close() {
    endwin();
}

fn draw_energy_bar(a: i32, main_wnd: &Window) {
    let mut col_pair;
    for i in (0..a).step_by(4) {
        if i > 100 {
            col_pair = 5;
        } else if i > 50 {
            col_pair = 2;
        } else if i > 25 {
            col_pair = 3;
        } else {
            col_pair = 4;
        }
        main_wnd.attron(COLOR_PAIR(col_pair));
        main_wnd.attron(A_BOLD);
        main_wnd.addch('/');
        main_wnd.attroff(A_BOLD);
        main_wnd.attroff(COLOR_PAIR(col_pair));
    }
}

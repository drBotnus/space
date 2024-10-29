use std::{thread::sleep, time::Duration};

use pancurses::{
    cbreak, curs_set, endwin, has_colors, init_pair, initscr, noecho, start_color, Input, Window,
    A_BOLD, COLOR_BLACK, COLOR_PAIR, COLOR_WHITE,
};

#[derive(Debug)]
struct Player {
    pos: IVec2d,
    disp_char: char,
}

#[derive(Debug)]
struct UVec2D {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct IVec2d {
    x: i32,
    y: i32,
}

pub fn init() -> Window {
    let wnd = initscr();
    cbreak();
    noecho();
    wnd.clear();
    wnd.refresh();
    wnd.keypad(true);
    wnd.nodelay(true);
    curs_set(0);
    if !has_colors() {
        endwin();
        panic!("ERROR: Terminal does not support color.");
    }
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    wnd.bkgd(COLOR_PAIR(1));
    wnd.attron(A_BOLD);
    wnd.draw_box(0, 0);
    wnd.attroff(A_BOLD);
    return wnd;
}

pub fn run(wnd: Window) {
    let mut player = Player {
        disp_char: '@',
        pos: IVec2d { x: 10, y: 5 },
    };
    wnd.mvaddch(player.pos.y, player.pos.x, player.disp_char);
    wnd.refresh();
    loop {
        if let Some(c) = wnd.getch() {
            match c {
                Input::Character('q') => break,
                Input::KeyUp | Input::Character('w') => player.pos.y -= 1,
                Input::KeyDown | Input::Character('s') => player.pos.y += 1,
                Input::KeyLeft | Input::Character('a') => player.pos.x -= 1,
                Input::KeyRight | Input::Character('d') => player.pos.x += 1,
                _ => (),
            }
            wnd.mvaddch(player.pos.y, player.pos.x, player.disp_char);
        }
        sleep(Duration::from_millis(10));
        wnd.refresh();
    }
}

pub fn close() {
    endwin();
}

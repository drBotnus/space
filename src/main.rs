mod game;
mod objects;
mod player;

fn main() {
    let (main_wnd, game_wnd, game_area, screen_area) = game::init();
    game::run(main_wnd, game_wnd, game_area, screen_area);
    game::close();
}

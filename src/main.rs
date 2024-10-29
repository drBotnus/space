mod game;

fn main() {
    let wnd = game::init();
    game::run(wnd);
    game::close();
}

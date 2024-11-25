mod game;
mod objects;
mod player;

fn main() {
    game::run(game::init());
    game::close();
}

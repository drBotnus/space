use pancurses::Input;

use crate::objects::Rect;

#[derive(Debug)]
pub struct Player {
    pub pos: (i32, i32),
    pub bounds: Rect,
    pub disp_char: char,
    pub energy: i32,
}

impl Default for Player {
    fn default() -> Self {
        let pos = (10, 10);
        Self {
            pos,
            bounds: Rect {
                offset: (pos.0 - 1, pos.1),
                bounds: (3, 2),
            },
            disp_char: 'o',
            energy: 100,
        }
    }
}

impl Player {
    pub fn parse_input(&mut self, c: Input, game_area: &Rect) {
        match c {
            Input::KeyUp | Input::Character('w') => {
                if self.pos.1 > game_area.top() {
                    self.pos.1 -= 1;
                }
            }
            Input::KeyDown | Input::Character('s') => {
                if self.pos.1 < game_area.bottom() + 1 {
                    self.pos.1 += 1
                }
            }
            Input::KeyLeft | Input::Character('a') => {
                if self.pos.0 > game_area.left() + 1 {
                    self.pos.0 -= 1;
                }
            }
            Input::KeyRight | Input::Character('d') => {
                if self.pos.0 < game_area.right() - 2 {
                    self.pos.0 += 1
                }
            }
            _ => (),
        }
    }
}

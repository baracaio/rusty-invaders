use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2, // deixa o personagem o meio da tela
            y: NUM_ROWS - 1, // como o eixo y é invertido, NUM_ROWS é o chão, daí precisa ser chão - 1 pra aparecer na tela
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = 'A';
    }
}
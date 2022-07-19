use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, current_frame: &Frame, last_frame: &Frame, force: bool) {
    if force {
        /**
        Retorna um Result, que pode ser um erro ou um sucesso.
        O Unwrap significa que não vou fazer nada com ele e se for um erro, simplesmente vai crashar
        Poderia usar um expect para dar uma mensagem de erro melhor na hora do crash.
        */
        stdout.queue(SetBackgroundColor(Color::DarkBlue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::White)).unwrap();
    }

    for (x, col) in current_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            // vem uma referência de referência, mas não sei exatamente de onde kakakaka
            // acho que fica na definição do frame
            // todo: ver de onde vem essa referência aqui
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}
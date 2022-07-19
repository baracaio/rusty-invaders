use std::error::Error;
use std::io;
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use invaders::frame::{Drawable, Frame, new_frame};
use invaders::player::Player;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    for item in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(item, &format!("audio/{}.wav", item));
    }

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?; // por que tem essa interrogação aqui?


    let mut player = Player::new();
    let mut current_frame: Frame;

    'mainloop: loop {
         current_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'mainloop;
                    },
                    _ => {}
                }
            }
        }
    }

    let drawables: Vec<&dyn Drawable> = vec![&player];

    for drawable in drawables {
        drawable.draw(&mut current_frame);
    }

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

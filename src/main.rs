use crossterm::{
    cursor::MoveTo,
    event::{self, poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, size, Clear, SetSize},
    ExecutableCommand, QueueableCommand,
};
use std::time::Duration;
use std::{
    io::{stdout, Write},
    thread,
};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}

fn main() {
    let _clean_up = CleanUp;
    let mut should_exit = false;
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    let (mut w, mut h) = size().unwrap();

    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap()
        .queue(MoveTo(0, 0))
        .unwrap();

    stdout.flush().unwrap();

    let mut ch: char = ' ';
    while !should_exit {
        if poll(Duration::from_secs_f32(0.5)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char(c) => {
                        if c == 'q' {
                            should_exit = true;
                        } else {
                            ch = c;
                        }
                    }
                    KeyCode::Esc => should_exit = true,
                    _ => (),
                },
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    let label = format!("{}x{}", w, h).to_string();

                    stdout
                        .queue(MoveTo(w / 2 - label.len() as u16 / 2, h / 2))
                        .unwrap()
                        .queue(Print(label))
                        .unwrap()
                        .queue(MoveTo(0, 0))
                        .unwrap();

                    stdout.flush().unwrap();

                    thread::sleep(Duration::from_secs(1));

                    stdout
                        .queue(MoveTo(0, nh / 2))
                        .unwrap()
                        .queue(Clear(terminal::ClearType::CurrentLine))
                        .unwrap()
                        .queue(MoveTo(0, 0))
                        .unwrap();

                    stdout.flush().unwrap();
                }
                _ => (),
            }
            stdout.execute(Print(ch)).unwrap();
        }
    }
    stdout.execute(Clear(terminal::ClearType::All)).unwrap();
}

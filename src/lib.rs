use std::io::{Read, Result, Write, stdout, Stdout};
use std::thread;
use std::time::Duration;
use crossterm::event::{read, Event, KeyEvent, KeyCode ,KeyModifiers};
use crossterm::style::{SetBackgroundColor, Color, Print, ResetColor, SetForegroundColor};
use crossterm::{ cursor, ExecutableCommand, terminal, QueueableCommand };
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

const BOARD_WIDTH: u16 = 10;
const BOARD_HEIGHT: u16 = 20;

// const BLOCK_I = [

// ]

pub struct Tet {
    score: usize,
    dropBlock: Stdout,
    x: u16,
    y: u16
}

impl Tet {
    pub fn new() -> Tet{
        let stdout = stdout();
        Tet {
            score: 0,
            dropBlock: stdout,
            x: 10,
            y: 0,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let mut stdout = stdout();
        enable_raw_mode().unwrap();

        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        'main: loop {
            stdout.queue(cursor::Hide)?;
            stdout
                .queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(0, 0))?;
                

            self.dropCurrentBlock();

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => break 'main,
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => self.moveLeft(),
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => self.moveRight(),
                _ => {}
            }
        }
        
        stdout.queue(cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn dropCurrentBlock(&mut self) {
        for _ in 0..BOARD_HEIGHT {
            self.moveBlock(0, 1);
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn moveBlock(&mut self, x: u16, y: u16) {
        self.x = self.x + x;
        self.y = self.y + y;
        self.currentMove()
    }
    fn moveLeft(&mut self) {
        self.x = self.x - 2;
        self.currentMove()
    }
    fn moveRight(&mut self) {
        self.x = self.x + 2;
        self.currentMove()
    }


    fn currentMove(&mut self) {
        self.dropBlock.queue(terminal::Clear(terminal::ClearType::FromCursorUp)).unwrap();
        self.dropBlock.execute(cursor::MoveTo(self.x, self.y)).unwrap();
        self.dropBlock.execute(SetBackgroundColor(Color::Red)).unwrap();
        self.dropBlock.execute(Print("  ")).unwrap();
        self.dropBlock.execute(ResetColor).unwrap();
    }
}
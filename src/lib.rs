use std::io::{ Result, stdout, Stdout };
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossterm::event::{ read, Event, KeyEvent, KeyCode ,KeyModifiers, poll, KeyEventState, KeyEventKind };
use crossterm::style::{ SetBackgroundColor, Color, Print, ResetColor };
use crossterm::{ cursor, ExecutableCommand, terminal, QueueableCommand };
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };

const BOARD_WIDTH: u16 = 10;
const BOARD_HEIGHT: u16 = 20;

enum KeyAction {
    End,
    Left,
    Right,
    Start,
}

// const BLOCK_I = [

// ]

struct Block {
    x: u16,
    y: u16
}

pub struct Tet {
    score: usize,
    block: Block,
    stdout: Stdout,
    action: Arc<Mutex<KeyAction>>
}

impl Tet {
    pub fn new() -> Tet{
        let stdout = stdout();
        let block = Block{ x: 10, y:0 };

        Tet {
            score: 0,
            block,
            stdout,
            action: Arc::new(Mutex::new(KeyAction::Start)),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let mut stdout = stdout();
        enable_raw_mode().unwrap();

        let action_clone = self.action.clone();

        thread::spawn(move || {
            loop {
                match read().unwrap() {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => {
                        let mut get_action_clone = action_clone.lock().unwrap();
                        *get_action_clone = KeyAction::End
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        ..
                    }) => {
                        let mut get_action_clone = action_clone.lock().unwrap();
                        *get_action_clone = KeyAction::Left
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        ..
                    }) => {
                        let mut get_action_clone = action_clone.lock().unwrap();
                        *get_action_clone = KeyAction::Right
                    },
                    _ => {}
                }
            }
        });

        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        'main: loop {
            stdout.queue(cursor::Hide)?;
            stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
            if self.block.y < BOARD_HEIGHT {
                self.move_block(0, 1);
                thread::sleep(Duration::from_millis(500));
            }
            
            let action_clone2 = self.action.clone();
            let mut action = action_clone2.lock().unwrap();

            match *action {
                KeyAction::End => break 'main,
                KeyAction::Left => {
                    *action = KeyAction::Start;
                    self.move_left();
                },
                KeyAction::Right => {
                    *action = KeyAction::Start;
                    self.move_right();
                },
                _ => {}
            }
        }
        
        stdout.queue(cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn drop_current_block(&mut self) {
        for _ in 0..BOARD_HEIGHT {
            self.move_block(0, 1);
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn check_can_move_bock(&mut self) -> bool {
        if self.block.y < BOARD_HEIGHT {
            return true
        }
        return false
    }

    fn move_block(&mut self, x: u16, y: u16) {
        self.block.x = self.block.x + x;
        self.block.y = self.block.y + y;
        self.current_move()
    }

    fn move_left(&mut self) {
        self.block.x = self.block.x - 2;
        self.current_move()
    }
    fn move_right(&mut self) {
        self.block.x = self.block.x + 2;
        self.current_move()
    }


    // stdout관련 이동 조작은 여기서만
    fn current_move(&mut self) {
        self.stdout.queue(terminal::Clear(terminal::ClearType::FromCursorUp)).unwrap();
        self.stdout.execute(cursor::MoveTo(self.block.x, self.block.y)).unwrap();
        self.stdout.execute(SetBackgroundColor(Color::Red)).unwrap();
        self.stdout.execute(Print("  ")).unwrap();
        self.stdout.execute(ResetColor).unwrap();
    }
}
use crate::game::Game;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::error::Error;

pub struct Editor {
    pub selected_row: usize,
    pub selected_col: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_row: 0,
            selected_col: 0,
        }
    }

    /// Returns a tuple containing:
    /// - A boolean indicating if the user confirmed (pressed Enter).
    /// - A message to display to the user.
    pub fn handle_input(&mut self, game: &mut Game) -> Result<(bool, String), Box<dyn Error>> {
        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                match event::read()? {
                    Event::Key(KeyEvent { code, .. }) => match code {
                        KeyCode::Left => {
                            if self.selected_col > 0 {
                                self.selected_col -= 1;
                            }
                            return Ok((false, "".to_string()));
                        }
                        KeyCode::Right => {
                            if self.selected_col < game.size - 1 {
                                self.selected_col += 1;
                            }
                            return Ok((false, "".to_string()));
                        }
                        KeyCode::Up => {
                            if let Some(val) = game.grid[self.selected_row][self.selected_col] {
                                let new_val = (val * 2).min(2048);
                                game.grid[self.selected_row][self.selected_col] = Some(new_val);
                            } else {
                                game.grid[self.selected_row][self.selected_col] = Some(2);
                            }
                            return Ok((false, "".to_string()));
                        }
                        KeyCode::Down => {
                            game.grid[self.selected_row][self.selected_col] = None;
                            return Ok((false, "".to_string()));
                        }
                        KeyCode::Enter => {
                            return Ok((true, "AI is making a move...".to_string()));
                        }
                        KeyCode::Esc => {
                            return Ok((false, "Exiting game.".to_string()));
                        }
                        _ => continue,
                    },
                    _ => continue,
                }
            } else {
                // Timeout: Optional feedback or continue waiting
                continue;
            }
        }
    }
}
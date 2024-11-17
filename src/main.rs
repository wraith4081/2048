mod ai;
mod game;
mod input;
mod ui;

use ai::Direction;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use input::Editor;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{
    error::Error,
    io::{self, Write},
};
use ui::draw_ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = setup_game()?;

    if game.game_over {
        println!("Failed to initialize the game. Exiting.");
        return Ok(());
    }

    println!("Initializing terminal...");
    match initialize_terminal() {
        Ok((mut terminal, guard)) => {
            let mut editor = Editor::new();
            let mut message = String::new();

            loop {
                draw_ui(&mut terminal, &game, &editor, &message)?;

                message = String::new();

                let (confirmed, input_message) = match editor.handle_input(&mut game) {
                    Ok(res) => res,
                    Err(e) => {
                        message = format!("Input Error: {}", e);
                        (false, message.clone())
                    }
                };
                message = input_message;

                if confirmed {
                    if game.game_over {
                        message = "Game Over!".to_string();
                    } else {
                        match game.ai_make_move() {
                            Some(direction) => {
                                let direction_clone = direction.clone();
                                apply_move(&mut game, direction.clone());
                                message = format!("AI selected move: {:?}", direction_clone);
                            }
                            None => {
                                message = "No possible moves. Game Over!".to_string();
                                game.game_over = true;
                            }
                        }
                    }
                }

                // Exit if pressed Esc or game is over
                if !confirmed || game.game_over {
                    break;
                }
            }

            match restore_terminal(terminal, guard) {
                Ok(_) => (),
                Err(e) => eprintln!("Error restoring terminal: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Terminal Initialization Error: {}", e);
            // Attempt to restore terminal in case of failure
            let _ = disable_raw_mode();
            let _ = execute!(
                io::stdout(),
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            return Err(Box::from(e));
        }
    }

    println!("Exiting game. Goodbye!");
    Ok(())
}

fn initialize_terminal() -> Result<(Terminal<CrosstermBackend<io::Stdout>>, RawModeGuard), Box<dyn Error>> {
    enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )
        .map_err(|e| format!("Failed to execute terminal commands: {}", e))?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).map_err(|e| format!("Failed to create terminal: {}", e))?;
    Ok((terminal, RawModeGuard {}))
}

fn restore_terminal<B: ratatui::backend::Backend + std::io::Write>(
    mut terminal: Terminal<B>,
    _guard: RawModeGuard,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode().map_err(|e| format!("Failed to disable raw mode: {}", e))?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
        .map_err(|e| format!("Failed to execute restore terminal commands: {}", e))?;
    terminal.show_cursor().map_err(|e| format!("Failed to show cursor: {}", e))?;
    Ok(())
}

struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if let Err(e) = disable_raw_mode() {
            eprintln!("Failed to disable raw mode on drop: {}", e);
        }
        if let Err(e) = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ) {
            eprintln!("Failed to leave alternate screen on drop: {}", e);
        }
    }
}

fn setup_game() -> Result<Game, Box<dyn Error>> {
    println!("Enter grid size (e.g., 4 for 4x4): ");
    io::stdout().flush()?; // Ensure the prompt is displayed

    let mut size_str = String::new();
    io::stdin().read_line(&mut size_str)?;

    let size: usize = match size_str.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("Invalid input. Defaulting to 4x4 grid.");
            4
        }
    };

    let game = Game::new(size);
    Ok(game)
}

fn apply_move(game: &mut Game, direction: Direction) {
    match direction {
        Direction::Left => {
            game.move_left();
        }
        Direction::Right => {
            game.move_right();
        }
        Direction::Up => {
            game.move_up();
        }
        Direction::Down => {
            game.move_down();
        }
    }

    if game.game_over {
        // The main loop will handle it
    }
}
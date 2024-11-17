use crate::game::Game;
use crate::input::Editor;
use ratatui::{
    layout::{Alignment, Constraint, Direction as LayoutDirection, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};
use std::error::Error;

pub fn draw_ui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    game: &Game,
    editor: &Editor,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| {
        let size = f.area();

        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3), // Header
                    Constraint::Min(1),    // Main (Game Grid)
                    Constraint::Length(3), // Footer
                ]
                    .as_ref(),
            )
            .split(size);

        let header = Paragraph::new(format!("2048 - Score: {}", game.score))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(Block::default().borders(Borders::ALL).title("2048"))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        let rows: Vec<Row> = game
            .grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let cells = row.iter().enumerate().map(|(j, cell)| {
                    let content = match cell {
                        Some(n) => n.to_string(),
                        None => " ".to_string(),
                    };
                    if i == editor.selected_row && j == editor.selected_col {
                        Cell::from(content)
                            .style(
                                Style::default()
                                    .fg(Color::Yellow)
                                    .bg(Color::Blue)
                                    .add_modifier(Modifier::BOLD),
                            )
                    } else {
                        Cell::from(content).style(Style::default().fg(Color::Yellow))
                    }
                });
                Row::new(cells).height(3)
            })
            .collect();

        let widths = vec![Constraint::Length(7); game.size];

        let table = Table::new(rows, widths)
            .block(Block::default().borders(Borders::ALL).title("Grid"))
            .column_spacing(1)
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol(">> ");

        f.render_widget(table, chunks[1]);

        let footer_chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints(
                [
                    Constraint::Length(1), // Instructions
                    Constraint::Length(1), // Message
                ]
                    .as_ref(),
            )
            .split(chunks[2]);

        let instructions = Paragraph::new("Use Left/Right to navigate | Up: +Value | Down: Clear | Enter: AI Move | Esc: Exit")
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL).title("Instructions"));

        let message_para = Paragraph::new(message)
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Message"));

        f.render_widget(instructions, footer_chunks[0]);
        f.render_widget(message_para, footer_chunks[1]);
    })?;
    Ok(())
}
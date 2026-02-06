use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveToColumn, MoveUp, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

use crate::git_operations::{get_log, CommitLog};

pub fn run_log() -> Result<(), String> {
    let logs = get_log()?;
    display_loop(logs).map_err(|e| format!("An error occurred during log: {}", e))?;
    return Ok(());
}

fn display_loop(logs: Vec<CommitLog>) -> Result<(), Error> {
    stdout().execute(Hide)?;
    enable_raw_mode()?;
    let formats = vec!["oneline", "short"];
    let mut format_index = 0;

    let mut out = stdout();
    let height = 1;

    let mut log_index = 0;

    loop {
        let event = read()?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                format_index = (format_index + 1) % formats.len();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                format_index = (format_index + formats.len() - 1) % formats.len();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            }) => {
                log_index = (log_index + logs.len() - 1) % logs.len();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            }) => {
                log_index = (log_index + 1) % logs.len();
            }
            _ => {}
        }

        execute!(out, MoveUp(height + 2), MoveToColumn(0))?;

        execute!(out, Clear(ClearType::CurrentLine)).unwrap();
        println!("{}", formats[format_index].green());
        execute!(out, Clear(ClearType::CurrentLine)).unwrap();
        println!("{}", logs[log_index]);
        execute!(out, Clear(ClearType::CurrentLine)).unwrap();
        println!(
            "{}",
            "[↑↓ to move, ←→ to switch format, type to filter]".blue()
        );

        out.flush()?;
    }
    execute!(out, MoveUp(1), MoveToColumn(0))?;
    execute!(out, Clear(ClearType::CurrentLine)).unwrap();
    stdout().execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

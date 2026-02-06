use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveToColumn, MoveUp, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

pub fn run_log() -> Result<(), String> {
    display_loop().map_err(|e| format!("An error occurred during log: {}", e))?;
    return Ok(());
}

fn display_loop() -> Result<(), Error> {
    stdout().execute(Hide)?;
    enable_raw_mode()?;
    let formats = vec!["oneline", "short"];
    let mut index = 0;

    let mut out = stdout();
    let height = 3;

    let mut counter = 0;
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
                index = (index + 1) % formats.len();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                index = (index + formats.len() - 1) % formats.len();
            }
            _ => {}
        }

        execute!(out, MoveUp(height+2), MoveToColumn(0))?;

        execute!(out, Clear(ClearType::CurrentLine)).unwrap();
        println!("{}", formats[index].green());
        for i in 0..height {
            execute!(out, Clear(ClearType::CurrentLine)).unwrap();
            println!("Line {}: {}", i + 1, counter + i);
        }
        execute!(out, Clear(ClearType::CurrentLine)).unwrap();
        println!("{}", "[↑↓ to move, ←→ to switch format, type to filter]".magenta());

        counter += 1;
        
        out.flush()?;
    }
    execute!(out, MoveUp(1), MoveToColumn(0))?;
    execute!(out, Clear(ClearType::CurrentLine)).unwrap();
    stdout().execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

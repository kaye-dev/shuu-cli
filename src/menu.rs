use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stderr = io::stderr();
        execute!(stderr, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stderr(), cursor::Show);
        let _ = disable_raw_mode();
    }
}

pub fn select_menu(title: &str, items: &[String], hint: &str) -> Option<usize> {
    let total = items.len();
    if total == 0 {
        return None;
    }

    let _guard = match RawModeGuard::new() {
        Ok(g) => g,
        Err(_) => return None,
    };

    let mut stderr = io::stderr();
    let mut selected: usize = 0;

    // Print title
    let _ = write!(stderr, "\r\n\x1b[1m{}\x1b[0m\r\n", title);

    let mut redraw = false;
    loop {
        if redraw {
            let _ = write!(stderr, "\r\x1b[{}A", total);
        }

        for (i, item) in items.iter().enumerate() {
            let _ = write!(stderr, "\x1b[2K");
            if i == selected {
                let _ = write!(stderr, "  \x1b[0;32m\u{25b6} {}\x1b[0m\r\n", item);
            } else {
                let _ = write!(stderr, "  \x1b[2m  {}\x1b[0m\r\n", item);
            }
        }
        let _ = write!(stderr, "\x1b[2K\x1b[2m  {}\x1b[0m", hint);
        let _ = stderr.flush();

        redraw = true;

        let event = match event::read() {
            Ok(e) => e,
            Err(_) => return None,
        };

        let key_event = match event {
            Event::Key(ke) if ke.kind == KeyEventKind::Press => ke,
            _ => continue,
        };

        let mut accept = false;

        match key_event.code {
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                let _ = write!(stderr, "\r\n");
                let _ = stderr.flush();
                return None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                selected = (selected + total - 1) % total;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                selected = (selected + 1) % total;
            }
            KeyCode::Enter => {
                accept = true;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                let _ = write!(stderr, "\r\n");
                let _ = stderr.flush();
                return None;
            }
            KeyCode::Char(c @ '1'..='9') => {
                let num = (c as usize) - ('1' as usize);
                if num < total {
                    selected = num;
                    accept = true;
                }
            }
            _ => {}
        }

        if accept {
            // Final redraw to show selection
            let _ = write!(stderr, "\r\x1b[{}A", total);
            for (i, item) in items.iter().enumerate() {
                let _ = write!(stderr, "\x1b[2K");
                if i == selected {
                    let _ = write!(stderr, "  \x1b[0;32m\u{25b6} {}\x1b[0m\r\n", item);
                } else {
                    let _ = write!(stderr, "  \x1b[2m  {}\x1b[0m\r\n", item);
                }
            }
            let _ = write!(stderr, "\x1b[2K\r\n");
            let _ = stderr.flush();
            return Some(selected);
        }
    }
}

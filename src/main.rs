use tui::layout::Constraint;
use array_macro::array;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Error},
    time::Duration,
    thread,
};
use tui::{backend::CrosstermBackend, Terminal};

mod board;
mod snake;
mod util;
mod movement;

fn main() -> Result<(), Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let width_constraints = array!(Constraint::Length(5); board::BOARD_WIDTH as usize);

    terminal.draw(|f| {
        let board = board::build_board_table(&width_constraints);
        f.render_widget(board, f.size());
    })?;
    thread::sleep(Duration::from_millis(4000));
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

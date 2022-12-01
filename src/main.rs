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

    // game state
    let mut direction = movement::Direction::Right;
    let mut snake = snake::init_snake_list();

    // todo replace with stop on failure state
    let interval = Duration::from_millis(1000);
    let max_iterations = 100;
    let mut current_iteration = 0; 
    let mut board_vector = board::init_board_vector(snake.clone());
    loop {
        if current_iteration > max_iterations {
            break
        }
        terminal.draw(|f| {
            let board = board::build_board_table(&width_constraints, board_vector.clone());
            f.render_widget(board, f.size());
        })?;
        match snake::make_iteration(snake.clone(), board_vector.clone(), direction.clone()) {
            Ok(new_state) => {
                snake = new_state.snake;
                board_vector = new_state.board;
            },
            Err(_) => break
        }
        thread::sleep(interval);
        current_iteration += 1;
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

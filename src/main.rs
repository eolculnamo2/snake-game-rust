use tui::layout::Constraint;
use std::{sync::mpsc, time::Instant};
use array_macro::array;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
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

enum GameEvent {
    Input(KeyEvent),
    Tick,
}
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
    let interval = Duration::from_millis(250);
    let mut board_vector = board::init_board_vector(snake.clone());

    // event management thraed
    let (dispatcher, subscriber) = mpsc::channel();
    let loop_time = interval;
    // let loop_time = Duration::from_millis(250);
    thread::spawn(move|| { 
        let mut last_tick = Instant::now();
        loop {
            let timeout = loop_time
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_micros(0));

            if event::poll(timeout).expect("poll works") {
                if let Event::Key(key) = event::read().expect("can read events") {
                    dispatcher.send(GameEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= loop_time {
                if dispatcher.send(GameEvent::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        }
    });

    loop {
        // todo, this sucks... have to clear our all pending ticks with while loop
        while let Ok(new_event) = subscriber.recv_timeout(Duration::from_millis(0)) {
            match new_event {
                GameEvent::Input(key_event) => {
                    match key_event.code {
                        KeyCode::Left if direction != movement::Direction::Right => direction = movement::Direction::Left,
                        KeyCode::Right if direction != movement::Direction::Left => direction = movement::Direction::Right,
                        KeyCode::Up if direction != movement::Direction::Down => direction = movement::Direction::Up,
                        KeyCode::Down if direction != movement::Direction::Up => direction = movement::Direction::Down,
                        _ => ()
                    }
                },
                GameEvent::Tick => ()
            }
        }

        match snake::make_iteration(snake.clone(), board_vector.clone(), direction.clone()) {
            Ok(new_state) => {
                if new_state.game_end.is_some() {
                    break
                }
                snake = new_state.snake;
                board_vector = new_state.board;
            },
            Err(_) => break
        }
        terminal.draw(|f| {
            let board = board::build_board_table(&width_constraints, board_vector.clone());
            f.render_widget(board, f.size());
        })?;
        thread::sleep(interval);
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    println!("Thanks for playing :)");
    Ok(())
}

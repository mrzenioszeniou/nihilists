pub mod economy;
pub mod nihilists;
pub mod state;
pub mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use state::State;
use std::time::{Duration, Instant};
use tui::{backend::CrosstermBackend, Terminal};

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut state = State::default();

    let day_duration = Duration::from_secs(1);
    let tick_duration = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        let mut should_quit = false;

        if crossterm::event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('Q') | KeyCode::Char('q') => should_quit = true,
                    KeyCode::Left => state.left(),
                    KeyCode::Up => state.up(),
                    KeyCode::Right => state.right(),
                    KeyCode::Down => state.down(),
                    _ => {}
                }
            }
        }

        let now = Instant::now();

        if now.duration_since(last_tick) >= day_duration {
            state = state.next();
            last_tick = Instant::now();
        }

        terminal.draw(|frame| ui::draw(frame, &state)).unwrap();

        if state.economy.extinct() || should_quit {
            disable_raw_mode().unwrap();
            execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
            terminal.clear().unwrap();
            terminal.show_cursor().unwrap();
            disable_raw_mode().unwrap();

            if state.economy.extinct() {
                println!("The human race is extinct! 🎉");
            }

            break;
        }

        std::thread::sleep(tick_duration);
    }
}

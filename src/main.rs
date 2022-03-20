pub mod economy;
pub mod nihilists;
pub mod ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use economy::Economy;
use nihilists::Nihilists;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};

fn main() {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut economy = Economy::default();
    let mut nihilists = Nihilists::default();

    loop {
        terminal
            .draw(|frame| ui::draw(frame, &economy, &nihilists))
            .unwrap();

        economy = economy.next(&nihilists);
        nihilists = nihilists.next();

        // println!("{}", state);
        // println!("{}", nihilists);

        if economy.extinct() {
            disable_raw_mode().unwrap();
            execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
            terminal.show_cursor().unwrap();

            println!("The human race is extinct! 🎉");
            break;
        }

        std::thread::sleep(Duration::from_secs(1));
        // std::thread::sleep(Duration::from_millis(50));
    }
}

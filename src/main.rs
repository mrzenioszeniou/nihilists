pub mod economy;
pub mod nihilists;

use economy::Economy;
use nihilists::Nihilists;
use std::time::Duration;

fn main() {
    let mut state = Economy::default();
    let nihilists = Nihilists::default();

    loop {
        state = state.next(&nihilists);

        println!("{}", state);

        if state.extinct() {
            println!("The human race is extinct! 🎉");
            break;
        }

        // std::thread::sleep(Duration::from_secs(1));
        std::thread::sleep(Duration::from_millis(50));
    }
}

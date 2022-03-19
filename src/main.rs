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

        println!("{:#?}", state);

        // std::thread::sleep(Duration::from_secs(5));
        std::thread::sleep(Duration::from_millis(50));
    }
}

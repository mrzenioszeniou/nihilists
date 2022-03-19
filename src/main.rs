mod economy;

use economy::State;
use std::time::Duration;

fn main() {
    let mut state = State::default();

    loop {
        state = state.next();

        println!("{:#?}", state);

        std::thread::sleep(Duration::from_secs(1));
    }
}

use crate::{economy::Economy, nihilists::Nihilists};

#[derive(Debug, Default)]
pub struct State {
    pub economy: Economy,
    pub nihilists: Nihilists,
    pub control: (usize, usize),
}

impl State {
    pub fn next(&self) -> Self {
        Self {
            economy: self.economy.next(&self.nihilists),
            nihilists: self.nihilists.next(),
            control: self.control,
        }
    }

    pub fn up(&mut self) {
        if self.control.1 > 0 {
            self.control.1 -= 1
        }
    }

    pub fn down(&mut self) {
        if self.control.1 < 2 {
            self.control.1 += 1
        }
    }

    pub fn left(&mut self) {
        if self.control.0 > 0 {
            self.control.0 -= 1
        }
    }

    pub fn right(&mut self) {
        if self.control.0 < 4 {
            self.control.0 += 1
        }
    }

    pub fn plus(&mut self) {

        // match self.control {
        //     (0, 1) =>
        // }
    }
}

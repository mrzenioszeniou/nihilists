use crate::{
    economy::{Building, Economy},
    nihilists::Nihilists,
};

#[derive(Debug, Default)]
pub struct State {
    pub economy: Economy,
    pub nihilists: Nihilists,
    pub control: (usize, usize),
    pub headlines: Vec<String>,
}

impl State {
    pub fn next(&self) -> Self {
        let mut nihilists = self.nihilists.clone();

        let (economy, mut headlines) = self.economy.next(&mut nihilists);

        headlines.extend(self.headlines.iter().cloned());

        Self {
            economy,
            nihilists: nihilists.next(),
            control: self.control,
            headlines,
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
        if self.nihilists.undercover == 0 {
            return;
        }

        match self.control {
            (0, 0) => return,
            (0, 1) => self.nihilists.recruiters += 1,
            (0, 2) => self.nihilists.hitmen += 1,
            (building, num) => {
                let building = Building::from(building - 1);

                let nihilists: &mut usize = match num {
                    0 => self.nihilists.agitators.get_mut(&building).unwrap(),
                    1 => self.nihilists.saboteurs.get_mut(&building).unwrap(),
                    2 => self.nihilists.embezzlers.get_mut(&building).unwrap(),
                    _ => unreachable!(),
                };

                *nihilists += 1;
            }
        }

        self.nihilists.undercover -= 1;
    }

    pub fn minus(&mut self) {
        match self.control {
            (0, 0) => return,
            (0, 1) if self.nihilists.recruiters > 0 => self.nihilists.recruiters -= 1,
            (0, 2) if self.nihilists.hitmen > 0 => self.nihilists.hitmen -= 1,
            (building, num) => {
                let building = Building::from(building - 1);

                let nihilists: &mut usize = match num {
                    0 => self.nihilists.agitators.get_mut(&building).unwrap(),
                    1 => self.nihilists.saboteurs.get_mut(&building).unwrap(),
                    2 => self.nihilists.embezzlers.get_mut(&building).unwrap(),
                    _ => unreachable!(),
                };

                if *nihilists > 0 {
                    *nihilists -= 1;
                } else {
                    return;
                }
            }
        }

        self.nihilists.undercover += 1;
    }
}

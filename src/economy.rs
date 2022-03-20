use std::fmt::Display;

use strum::{AsRefStr, EnumIter, IntoEnumIterator};

use crate::nihilists::Nihilists;

const FOOD_TO_BABIES: f32 = 0.05;
const FOOD_TO_DEATHS: f32 = 0.35;
const EFFICIENCY_STEP: f32 = 0.001;
const STORAGE_STEP: usize = 1;
const HOUSING_STEP: usize = 1;

#[derive(Debug)]
pub struct Economy {
    pub food: usize,
    pub wood: usize,
    pub stone: usize,
    pub iron: usize,
    pub storage: usize,

    pub efficiency: f32,

    pub population: usize,
    pub population_cap: usize,

    pub day: usize,
}

impl Display for Economy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-------------------------")?;
        writeln!(f, "Day {} ({})", self.day, Season::from(self.day).as_ref())?;
        writeln!(f, "-------------------------")?;
        writeln!(f, "🍖 {:>5}", self.food)?;
        writeln!(f, "🪵 {:>5}", self.wood)?;
        writeln!(f, "🪨 {:>5}", self.stone)?;
        writeln!(f, "🪙 {:>5}", self.iron)?;
        writeln!(f)?;
        writeln!(f, "📦 {:>5}", self.storage)?;
        writeln!(f)?;
        writeln!(f, "🏭 {:>4.1}%", (self.efficiency - 1.0) * 100.0)?;
        writeln!(f)?;
        writeln!(f, "👨‍👩‍👧‍👦 {:>5}", self.population)?;
        writeln!(f, "🛖 {:>5}", self.population_cap)?;
        writeln!(f, "-------------------------\n\n\n")
    }
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            food: 20,
            wood: 0,
            stone: 0,
            iron: 0,
            storage: 200,

            efficiency: 1.0,

            population: 10,
            population_cap: 20,

            day: 0,
        }
    }
}

impl Economy {
    pub fn extinct(&self) -> bool {
        self.population < 2
    }

    pub fn next(&self, nils: &mut Nihilists) -> (Self, Vec<String>) {
        // Headlines
        let mut headlines = vec![];

        // Get the standard production per citizen based on the season
        let production = Season::from(self.day).production();

        let population_f = self.population as f32;

        // Multiply it by the population and efficiency modifier and embezzlement modifier
        let mut food = (production[0]
            * population_f
            * self.efficiency
            * nils.agitator_modifier(&Building::Hunting)) as usize;
        let mut wood = (production[1]
            * population_f
            * self.efficiency
            * nils.agitator_modifier(&Building::Lumberyard)) as usize;
        let mut stone = (production[2]
            * population_f
            * self.efficiency
            * nils.agitator_modifier(&Building::Quarry)) as usize;
        let mut iron = (production[3]
            * population_f
            * self.efficiency
            * nils.agitator_modifier(&Building::Mine)) as usize;

        // Sabotage
        if nils.sabotaged(&Building::Hunting) {
            headlines.push(format!(
                "[Day {:>3}] Hunting accident involving a bad arrows suspends meat production!",
                self.day
            ));
            food = 0;
        }
        if nils.sabotaged(&Building::Lumberyard) {
            headlines.push(format!(
                "[Day {:>3}] Lumberyard out of commission - all axes mysteriously blunt!",
                self.day
            ));
            wood = 0;
        }
        if nils.sabotaged(&Building::Quarry) {
            headlines.push(format!(
                "[Day {:>3}] Quarry flooded! Investigation into faulty pressure regulators",
                self.day
            ));
            stone = 0;
        }
        if nils.sabotaged(&Building::Mine) {
            headlines.push(format!(
                "[Day {:>3}] Cavein at the mine! Suspicious explosion at the lower levels.",
                self.day
            ));

            iron = 0;
        }

        // Add the previous stockpiles
        food += self.food;
        wood += self.wood;
        stone += self.stone;
        iron += self.iron;

        // Embezzle
        food = (food as f32 * nils.embezzlement(&Building::Hunting)) as usize;
        wood = (wood as f32 * nils.embezzlement(&Building::Lumberyard)) as usize;
        stone = (stone as f32 * nils.embezzlement(&Building::Quarry)) as usize;
        iron = (iron as f32 * nils.embezzlement(&Building::Mine)) as usize;

        // Feed the masses
        let mut population = if food >= self.population {
            // Feed the current population
            food -= self.population;

            // Babies!
            let space = self.population_cap - self.population;
            let births = std::cmp::min(space, (food as f32 * FOOD_TO_BABIES) as usize);
            food -= births;

            self.population + births
        } else {
            let missing_food = self.population - food;
            food = 0;
            self.population - (missing_food as f32 * FOOD_TO_DEATHS).ceil() as usize
        };

        // Hitmen TODO: Headline
        let hits = nils.hit();
        population = population.saturating_sub(hits);
        if hits > 0 {
            headlines.push(format!(
                "[Day {:>3}] {} found dead this morning. Authorities launch investigation.",
                self.day, hits
            ))
        }

        // Increase efficiency
        let mut efficiency = self.efficiency;
        if iron > 0 {
            efficiency += EFFICIENCY_STEP;
            iron -= 1;
        }

        // Increase storage size
        let mut storage_size = self.storage;
        if stone > 0 {
            storage_size += STORAGE_STEP;
            stone -= 1;
        }

        // Increase population_cap
        let mut population_cap = self.population_cap;
        if wood > 0 {
            population_cap += HOUSING_STEP;
            wood -= 1;
        }

        if headlines.is_empty() {
            headlines.push(format!("[Day {:>3}] Another lovely day!", self.day))
        }

        (
            Self {
                food: std::cmp::min(food, self.storage),
                wood: std::cmp::min(wood, self.storage),
                stone: std::cmp::min(stone, self.storage),
                iron: std::cmp::min(iron, self.storage),
                storage: storage_size,
                efficiency,
                population,
                population_cap,
                day: self.day + 1,
            },
            headlines,
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resource {
    Food,  //    Excess => +population |  Lack => -population
    Wood,  //    Excess => +pop_cap    |  Lack =>
    Stone, //    Excess => +storage    |  Lack =>
    Iron,  //    Excess => +efficiency |  Lack =>
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Building {
    Hunting,
    Lumberyard,
    Quarry,
    Mine,
}

impl std::fmt::Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lumberyard => write!(f, "🪓 Lumberyard 🪵"),
            Self::Quarry => write!(f, "⛏ Quarry 🪨"),
            Self::Mine => write!(f, "🔨 Mine 🪙"),
            Self::Hunting => write!(f, "🏹 Hunter's Lodge 🍖"),
        }
    }
}

impl From<usize> for Building {
    fn from(from: usize) -> Self {
        match from {
            0 => Self::Hunting,
            1 => Self::Lumberyard,
            2 => Self::Quarry,
            3 => Self::Mine,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, EnumIter, AsRefStr)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn length(&self) -> usize {
        match self {
            Self::Spring => 10,
            Self::Summer => 23,
            Self::Autumn => 15,
            Self::Winter => 20,
        }
    }

    // [Food, Wood, Stone, Iron]
    pub fn production(&self) -> [f32; 4] {
        match self {
            Self::Spring => [1.5, 0.2, 0.1, 0.0],
            Self::Summer => [3.5, 0.0, 0.0, 0.0],
            Self::Autumn => [1.5, 0.0, 0.1, 0.0],
            Self::Winter => [0.5, 0.0, 0.0, 0.1],
        }
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.as_ref(),
            match self {
                Self::Spring => "🌸",
                Self::Summer => "🌞",
                Self::Autumn => "🍁",
                Self::Winter => "❄️",
            }
        )
    }
}

impl From<usize> for Season {
    fn from(mut day: usize) -> Self {
        let year: usize = Self::iter().map(|s| s.length()).sum();

        day %= year;

        for season in Self::iter() {
            if day < season.length() {
                return season;
            } else {
                day -= season.length();
            }
        }

        panic!("Impossible");
    }
}

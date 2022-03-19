use crate::nihilists::Nihilists;

const FOOD_TO_BABIES: f32 = 0.05;
const FOOD_TO_DEATHS: f32 = 0.5;
const EFFICIENCY_STEP: f32 = 0.001;
const STORAGE_STEP: usize = 1;
const HOUSING_STEP: usize = 1;

#[derive(Debug)]
pub struct Economy {
    food: usize,
    wood: usize,
    stone: usize,
    iron: usize,
    storage: usize,

    efficiency: f32,

    population: usize,
    population_cap: usize,

    day: usize,
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            food: 20,
            wood: 0,
            stone: 0,
            iron: 0,
            storage: 50,

            efficiency: 1.0,

            population: 10,
            population_cap: 20,

            day: 0,
        }
    }
}

impl Economy {
    pub fn next(&self, _: &Nihilists) -> Self {
        // Get the standard production per citizen based on the season
        let production = Season::from(self.day).production();

        // Multiply it by the population and efficiency modifier
        let mut food = ((production[0] * self.population) as f32 * self.efficiency) as usize;
        let mut wood = ((production[1] * self.population) as f32 * self.efficiency) as usize;
        let mut stone = ((production[2] * self.population) as f32 * self.efficiency) as usize;
        let mut iron = ((production[3] * self.population) as f32 * self.efficiency) as usize;

        // Add the previous stockpiles
        food += self.food;
        wood += self.wood;
        stone += self.stone;
        iron += self.iron;

        // Feed the masses
        let population = if food >= self.population {
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
            self.population - (missing_food as f32 * FOOD_TO_DEATHS) as usize
        };

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
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resource {
    Food,  //    Excess => +population |  Lack => -population
    Wood,  //    Excess => +pop_cap    |  Lack =>
    Stone, //    Excess => +storage    |  Lack =>
    Iron,  //    Excess => +efficiency |  Lack =>
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Building {
    Lumberyard,
    Quarry,
    Foundry,
    Hunting,
}

pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    // [Food, Wood, Stone, Iron]
    pub fn production(&self) -> [usize; 4] {
        match self {
            Self::Spring => [1, 1, 0, 0],
            Self::Summer => [2, 0, 0, 0],
            Self::Autumn => [1, 0, 1, 0],
            Self::Winter => [0, 0, 0, 1],
        }
    }
}

impl From<usize> for Season {
    fn from(day: usize) -> Self {
        match day % 360 {
            0..=89 => Self::Spring,
            90..=179 => Self::Summer,
            180..=269 => Self::Autumn,
            270..=359 => Self::Winter,
            _ => panic!("Wat?"),
        }
    }
}

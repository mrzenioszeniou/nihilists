use std::collections::BTreeMap;

const BOOMER_METER_SIZE: usize = 20;

#[derive(Debug)]
pub struct State {
    resources: BTreeMap<Resource, usize>,
    placements: BTreeMap<Building, usize>,
    boomer_meter: usize,
    population: usize,
    day: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            resources: BTreeMap::from([
                (Resource::Food, 0),
                (Resource::Wood, 0),
                (Resource::Stone, 0),
                (Resource::Iron, 0),
            ]),
            placements: Default::default(),
            boomer_meter: 0,
            population: 2,
            day: 0,
        }
    }
}

impl State {
    fn unemployed(&self) -> usize {
        self.population - self.placements.values().sum::<usize>()
    }

    pub fn next(&self) -> Self {
        let resources = Season::from(self.day)
            .production()
            .into_iter()
            .map(|(res, cnt)| {
                let curr = self.resources.get(&res).unwrap();
                (res, curr + cnt)
            })
            .collect();

        let boomer_meter = self.boomer_meter + self.unemployed() / 2;

        Self {
            resources,
            placements: self.placements.clone(),
            boomer_meter: boomer_meter % BOOMER_METER_SIZE,
            population: self.population + boomer_meter / BOOMER_METER_SIZE,
            day: self.day + 1,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resource {
    Wood,
    Food,
    Stone,
    Iron,
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
    pub fn production(&self) -> BTreeMap<Resource, usize> {
        BTreeMap::from(match self {
            Self::Spring => [
                (Resource::Food, 2),
                (Resource::Wood, 4),
                (Resource::Stone, 0),
                (Resource::Iron, 1),
            ],
            Self::Summer => [
                (Resource::Food, 4),
                (Resource::Wood, 2),
                (Resource::Stone, 1),
                (Resource::Iron, 0),
            ],
            Self::Autumn => [
                (Resource::Food, 1),
                (Resource::Wood, 0),
                (Resource::Stone, 4),
                (Resource::Iron, 2),
            ],
            Self::Winter => [
                (Resource::Food, 0),
                (Resource::Wood, 1),
                (Resource::Stone, 2),
                (Resource::Iron, 4),
            ],
        })
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

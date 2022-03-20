use rand::{prelude::ThreadRng, Rng};
use std::{collections::HashMap, fmt::Display};
use strum::IntoEnumIterator;

use crate::economy::Building;

const AGITATOR_MODIFIER: f32 = 0.90;
const SABOTEUR_LIKELIHOOD: f32 = 0.01;
const EMBEZZLER_MODIFIER: f32 = 0.99;
const RECRUITER_LIKELIHOOD: f32 = 0.01;
const HITMAN_LIKELIHOOD: f32 = 0.01;
const EFFICIENCY_STEP: f32 = 0.001;

#[derive(Clone, Debug)]
pub struct Nihilists {
    pub agitators: HashMap<Building, usize>,
    pub saboteurs: HashMap<Building, usize>,
    pub embezzlers: HashMap<Building, usize>,

    pub undercover: usize,
    pub recruiters: usize,
    pub hitmen: usize,

    pub efficiency: f32,

    rng: ThreadRng,
}

impl Display for Nihilists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-- Nihilists ------------")?;
        writeln!(f, "   Agitators Saboteurs Embezzlers")?;
        writeln!(
            f,
            "🍖 {:<9} {:<9} {}",
            self.agitators.get(&Building::Hunting).unwrap(),
            self.saboteurs.get(&Building::Hunting).unwrap(),
            self.embezzlers.get(&Building::Hunting).unwrap(),
        )?;
        writeln!(
            f,
            "🪵 {:<9} {:<9} {}",
            self.agitators.get(&Building::Lumberyard).unwrap(),
            self.saboteurs.get(&Building::Lumberyard).unwrap(),
            self.embezzlers.get(&Building::Lumberyard).unwrap(),
        )?;
        writeln!(
            f,
            "🪨 {:<9} {:<9} {}",
            self.agitators.get(&Building::Quarry).unwrap(),
            self.saboteurs.get(&Building::Quarry).unwrap(),
            self.embezzlers.get(&Building::Quarry).unwrap(),
        )?;
        writeln!(
            f,
            "🪙 {:<9} {:<9} {}",
            self.agitators.get(&Building::Mine).unwrap(),
            self.saboteurs.get(&Building::Mine).unwrap(),
            self.embezzlers.get(&Building::Mine).unwrap(),
        )?;
        writeln!(f)?;
        writeln!(f, "🕵️ {:>5}", self.undercover)?;
        writeln!(f, "🤝 {:>5}", self.recruiters)?;
        writeln!(f, "🔪 {:>5}", self.hitmen)?;
        writeln!(f)?;
        writeln!(f, "🏭 {:>4.1}%", (self.efficiency - 1.0) * 100.0)?;
        writeln!(f, "-------------------------")
    }
}

impl Nihilists {
    pub fn next(&self) -> Self {
        let mut cloned = self.clone();

        if RECRUITER_LIKELIHOOD * cloned.recruiters as f32 * cloned.efficiency
            > cloned.rng.gen::<f32>()
        {
            cloned.undercover += 1;
        }

        cloned.efficiency += cloned.undercover as f32 * EFFICIENCY_STEP;

        cloned
    }

    pub fn agitator_modifier(&self, building: &Building) -> f32 {
        (AGITATOR_MODIFIER * self.efficiency).powi(*self.agitators.get(building).unwrap() as i32)
    }

    pub fn sabotaged(&mut self, building: &Building) -> bool {
        let sabotage_likelihood =
            *self.saboteurs.get(building).unwrap() as f32 * SABOTEUR_LIKELIHOOD * self.efficiency;

        self.rng.gen::<f32>() < sabotage_likelihood
    }

    pub fn embezzlement(&self, building: &Building) -> f32 {
        (EMBEZZLER_MODIFIER * self.efficiency).powi(*self.embezzlers.get(building).unwrap() as i32)
    }

    pub fn hit(&mut self) -> usize {
        let hit_likelihood = HITMAN_LIKELIHOOD * self.efficiency;

        (0..self.hitmen)
            .filter(|_| self.rng.gen::<f32>() < hit_likelihood)
            .count()
    }
}

impl Default for Nihilists {
    fn default() -> Self {
        Self {
            agitators: Building::iter().map(|b| (b, 0)).collect(),
            saboteurs: Building::iter().map(|b| (b, 0)).collect(),
            embezzlers: Building::iter().map(|b| (b, 0)).collect(),

            recruiters: 0,
            hitmen: 0,

            undercover: 10,
            efficiency: 1.0,
            rng: rand::thread_rng(),
        }
    }
}

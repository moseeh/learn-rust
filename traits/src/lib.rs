use std::fmt;
use std::fmt::Display;


#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub strength: f64,
    pub score: i32,
    pub money: i32,
    pub weapons: Vec<String>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        4.0 * self.weight_in_kg
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat_kg = self.fat_content * self.weight_in_kg;
        let protein_kg = self.weight_in_kg - fat_kg;

        fat_kg * 9.0 + protein_kg * 4.0
    }
}

impl Player {
    pub fn eat<F: Food>(&mut self, food: F) {
        self.strength += food.gives();
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            self.strength, self.score, self.money
        )?;
        writeln!(f, "Weapons: {:?}", self.weapons)
    }
}

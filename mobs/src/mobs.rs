// mob.rs

// Importing the `boss` and `member` modules
pub mod boss;
pub mod member;

// Importing `Boss`, `Member`, and `Role` from the respective modules
use boss::Boss;
use member::{Member, Role};

// Define the `Mob` struct representing a mafia organization
#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,       // Name of the mob
    pub boss: Boss,         // The boss of the mob
    pub members: Vec<Member>, // List of members in the mob
    pub cities: Vec<(String, u8)>, // Cities controlled by the mob, each with a value (e.g., importance or population)
    pub wealth: u32,        // Wealth accumulated by the mob
}

impl Mob {
    // Method to recruit a new member into the mob
    pub fn recruit(&mut self, member_name: &str, member_age: u8) {
        // Create a new member with the role `Associate` and add them to the `members` list
        self.members
            .push(Member::new(member_name, Role::Associate, member_age));
    }

    // Method to initiate an attack on another mob
    pub fn attack(&mut self, target: &mut Mob) {
        // If the current mob is stronger (calculated by `calculate_power`), remove one member from the target mob
        if calculate_power(self) > calculate_power(target) {
            target.members.pop(); // Remove a member from the target mob
        } else {
            self.members.pop(); // Remove a member from the current mob
        }

        // If the current mob's members are all gone, transfer cities and wealth to the target mob
        if self.members.is_empty() {
            switch_cities(target, self); // Transfer cities from current mob to the target mob
            target.wealth += self.wealth; // Transfer wealth to the target mob
            self.cities = vec![];        // Empty the cities of the current mob
            self.wealth = 0;             // Set current mob's wealth to 0
        } else if target.members.is_empty() {
            // If the target mob's members are all gone, transfer cities and wealth to the current mob
            switch_cities(self, target); // Transfer cities from target mob to the current mob
            self.wealth += target.wealth; // Transfer wealth to the current mob
            target.cities = vec![];      // Empty the cities of the target mob
            target.wealth = 0;           // Set target mob's wealth to 0
        }
    }

    // Method to steal wealth from another mob
    pub fn steal(&mut self, target: &mut Mob, value: u32) {
        // If the target mob has enough wealth, steal the specified value
        if target.wealth >= value {
            self.wealth += value;   // Add the stolen wealth to the current mob
            target.wealth -= value; // Subtract the stolen wealth from the target mob
        } else {
            // If the target mob doesn't have enough wealth, steal all of their wealth
            self.wealth += target.wealth;
            target.wealth = 0; // Set target mob's wealth to 0
        }
    }

    // Method to conquer a city from other mobs
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, wanted_city: String, value: u8) {
        // If no mob already controls the wanted city, add it to the current mob's cities
        if !mobs
            .into_iter()
            .any(|ele| ele.cities.iter().any(|(city, _)| city == &wanted_city))
        {
            self.cities.push((wanted_city, value)); // Add the new city to the current mob's cities
        }
    }
}

// Helper function to calculate the "power" of a mob based on its members' roles
fn calculate_power(mob: &Mob) -> usize {
    let mut result = 0;
    for member in &mob.members {
        // Add power points based on each member's role
        match member.role {
            Role::Underboss => result += 4,    // Underboss adds 4 points
            Role::Caporegime => result += 3,   // Caporegime adds 3 points
            Role::Soldier => result += 2,      // Soldier adds 2 points
            Role::Associate => result += 1,    // Associate adds 1 point
        }
    }
    result // Return the total power
}

// Helper function to transfer cities from one mob to another
fn switch_cities(winner: &mut Mob, loser: &Mob) {
    for city in &loser.cities {
        // Add each city from the loser to the winner's list of cities
        winner.cities.push(city.clone());
    }
}
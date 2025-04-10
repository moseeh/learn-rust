// Define an enum `Role` representing different roles in a mafia organization
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,    // Highest role
    Soldier,      // Mid-level role
    Caporegime,   // Lower mid-level role
    Associate,    // Entry-level role
}

// Define the `Member` struct, which represents a member of the mafia organization
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String, // Member's name
    pub role: Role,   // Member's role
    pub age: u8,      // Member's age
}

impl Member {
    // Method to promote a member to the next higher role
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,       // Promote Associate to Soldier
            Role::Soldier => self.role = Role::Caporegime,      // Promote Soldier to Caporegime
            Role::Caporegime => self.role = Role::Underboss,    // Promote Caporegime to Underboss
            _ => {} // If already at Underboss, do nothing
        }
    }

    // Constructor method to create a new instance of `Member`
    pub fn new(name: &str, role: Role, age: u8) -> Self {
        Self {
            name: name.to_string(), // Convert the `name` to a `String` and assign it to the `name` field
            role, // Assign the provided `role` to the `role` field
            age,  // Assign the provided `age` to the `age` field
        }
    }
}
// Define the `Boss` struct with two fields: `name` (String) and `age` (u8)
#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

impl Boss {
    // Constructor method to create a new instance of `Boss`
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(), // Convert the `name` to a `String` and assign it to the `name` field
            age, // Assign the provided `age` to the `age` field
        }
    }
}
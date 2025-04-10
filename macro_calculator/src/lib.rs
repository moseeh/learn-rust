extern crate json;
use json::object;

// Struct to hold nutritional information for a food item
pub struct Food {
    #[allow(dead_code)] // Allows the 'name' field to be unused without warning
    pub name: String, // Name of the food
    pub calories: [String; 2], // Energy values in kJ and kcal
    pub proteins: f64, // Amount of protein per portion
    pub fats: f64, // Amount of fats per portion
    pub carbs: f64, // Amount of carbs per portion
    pub nbr_of_portions: f64, // Number of portions consumed
}

// Function to calculate total macros for a list of foods
pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    // Initialize total values
    let (mut cals, mut prot, mut carbs, mut fats) = (0.0, 0.0, 0.0, 0.0);

    // Iterate through each food item
    for food in foods {
        // Extract numeric kcal value from calories[1], parse to f64
        let cal = &food.calories[1]
            .to_string() // Convert String to String again (clone)
            .split("kcal") // Split by "kcal"
            .collect::<Vec<&str>>()[0] // Take the numeric part before "kcal"
            .to_string() // Convert &str to String
            .parse::<f64>() // Parse String to f64
            .unwrap(); // Panic if parse fails

        // Accumulate totals scaled by number of portions
        cals += cal * food.nbr_of_portions;
        prot += food.proteins * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    // Return totals rounded to 2 decimal places as JSON object
    object! {
        "cals": (cals * 100.0).round() / 100.0,
        "carbs": (carbs * 100.0).round() / 100.0,
        "proteins": (prot * 100.0).round() / 100.0,
        "fats": (fats * 100.0).round() / 100.0,
    }
}

#[cfg(test)] // Test module
mod test {
    use super::*; // Import everything from parent module

    // Test macro calculation with example values
    #[test]
    fn testing_macros_values() {
        let a = Food {
            name: "light milk".to_string(),
            calories: ["148kJ".to_string(), "35kcal".to_string()],
            proteins: 3.5,
            fats: 0.1,
            carbs: 5.0,
            nbr_of_portions: 0.7,
        };
        let b = Food {
            name: "oat cookies".to_string(),
            calories: ["1996kJ".to_string(), "477kcal".to_string()],
            proteins: 8.2,
            fats: 21.0,
            carbs: 60.4,
            nbr_of_portions: 1.2,
        };

        let macros = calculate_macros(vec![a, b]);

        // Check expected total macro values
        assert_eq!(macros["cals"], 596.9);
        assert_eq!(macros["carbs"], 75.98);
        assert_eq!(macros["proteins"], 12.29);
        assert_eq!(macros["fats"], 25.27);
    }

    // Test when no food items are provided
    #[test]
    fn testing_no_food() {
        let macros = calculate_macros(vec![]);

        // Expect all values to be zero
        assert_eq!(macros["cals"], 0.0);
        assert_eq!(macros["carbs"], 0.0);
        assert_eq!(macros["proteins"], 0.0);
        assert_eq!(macros["fats"], 0.0);
    }

    // Test with larger food values and portions
    #[test]
    fn big_values() {
        let macros = calculate_macros(vec![
            Food {
                name: "big mac".to_string(),
                calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
                proteins: 27.0,
                fats: 26.0,
                carbs: 41.0,
                nbr_of_portions: 2.0,
            },
            Food {
                name: "pizza margherita".to_string(),
                calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ]);

        // Check expected total macro values for big inputs
        assert_eq!(macros["cals"].as_f64().unwrap(), 2777.39);
        assert_eq!(macros["carbs"].as_f64().unwrap(), 322.44);
        assert_eq!(macros["proteins"].as_f64().unwrap(), 122.06);
        assert_eq!(macros["fats"].as_f64().unwrap(), 106.93);
    }
}
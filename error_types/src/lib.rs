use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        // Format current date and time without external dependencies
        let now = SystemTime::now();
        let datetime = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        
        // This is a simple implementation to get "YYYY-MM-DD HH:MM:SS" format
        // In a real application, you would use a more robust approach
        let timestamp = datetime.as_secs();
        let secs = timestamp % 60;
        let mins = (timestamp / 60) % 60;
        let hours = (timestamp / 3600) % 24;
        let days_since_epoch = timestamp / 86400;
        
        // Simplified date calculation (doesn't handle leap years perfectly)
        let mut year = 1970;
        let mut days_remaining = days_since_epoch;
        
        // Advance years until we have less than 365/366 days left
        loop {
            let days_in_year = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                366
            } else {
                365
            };
            
            if days_remaining < days_in_year {
                break;
            }
            
            days_remaining -= days_in_year;
            year += 1;
        }
        
        // Determine month and day
        let days_in_month = [31, 
                            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
                            31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        
        let mut month = 0;
        let mut day = days_remaining as u8 + 1; // 1-indexed days
        
        for (i, &days) in days_in_month.iter().enumerate() {
            if day <= days {
                month = i + 1; // 1-indexed months
                break;
            }
            day -= days;
        }
        
        // Format the date string
        let date_str = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hours, mins, secs
        );
        
        FormError {
            form_values: (field_name, field_value),
            date: date_str,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Form { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        // Check if name is empty
        if self.name.is_empty() {
            return Err(FormError::new(
                "name", 
                self.name.clone(), 
                "Username is empty"
            ));
        }

        // Check password length
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long"
            ));
        }

        // Check password composition
        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| c.is_ascii() && !c.is_ascii_alphanumeric());

        if !(has_letter && has_digit && has_symbol) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols"
            ));
        }

        Ok(())
    }
}
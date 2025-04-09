use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Local};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let current_time = Local::now();
        let formatted_date = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
        
        FormError {
            form_values: (field_name, field_value),
            date: formatted_date,
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
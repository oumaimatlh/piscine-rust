
use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError {
            form_values: (field_name, field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
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
    pub fn validate(&self) -> Result<(), FormError> {
        let mut check_alpha = false;
        let mut check_numeric = false;
        let mut check_symbols = false;

        for c in self.password.chars() {
            if c.is_ascii_alphabetic() {
                check_alpha = true;
            } else if c.is_ascii_digit() {
                check_numeric = true;
            } else if !c.is_ascii_alphanumeric() {
                check_symbols = true;
            }
        }

        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        if !check_alpha || !check_numeric || !check_symbols {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}

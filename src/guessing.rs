use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: Option<u32>,
    secret_number: u32,
}

impl Guess {
    pub fn new() -> Self {
        Guess {
            value: Option::None,
            secret_number: rand::thread_rng().gen_range(1..=100),
        }
    }

    pub fn set_value(&mut self, value: u32) {
        if value < 1 || value > 100 {
            panic!("The provided value is invalid. It must be between 1 and 100.")
        }

        self.value = Option::Some(value);
    }

    pub fn value(&self) -> u32 {
        if let Option::Some(value) = self.value {
            return value;
        }

        panic!("Cannot get value. Reason: value not set.");
    }

    pub fn compare_guess(&self) -> Ordering {
        if let Option::Some(value) = self.value {
            return value.cmp(&self.secret_number);
        }

        panic!("Guess not set, thus cannot be compared.");
    }
}

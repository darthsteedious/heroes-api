use std::error::Error;
use std::fmt;
use serde::export::Formatter;

#[derive(Debug)]
pub struct AlreadyAssignedError(String);

impl AlreadyAssignedError {
    pub fn new(msg: &str) -> AlreadyAssignedError {
        AlreadyAssignedError(String::from(msg))
    }

    pub fn boxed(msg: &str) -> Box<dyn Error> {
        Box::new(AlreadyAssignedError::new(msg))
    }
}

impl fmt::Display for AlreadyAssignedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Property {} has already been assigned and cannot be updated", &self.0)
    }
}

impl Error for AlreadyAssignedError {}

#[derive(Debug)]
pub struct HeroNotFound(pub i32);

impl HeroNotFound {
    pub fn new(id: i32) -> HeroNotFound {
        HeroNotFound(id)
    }

    pub fn boxed(id: i32) -> Box<dyn Error> {
        Box::new(HeroNotFound::new(id))
    }
}

impl fmt::Display for HeroNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Could not find a hero with id {}", self.0)
    }
}

impl Error for HeroNotFound {

}
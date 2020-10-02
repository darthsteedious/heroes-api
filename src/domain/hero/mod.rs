pub mod repository;
mod stats;
pub mod errors;

use serde::{Serialize, Deserialize};
use super::Result;
use errors::AlreadyAssignedError;
use std::cmp::{PartialEq, Eq};

#[derive(Serialize, Deserialize)]
pub struct HeroDto {
    pub id: i32,
    pub name: String,
}

impl HeroDto {
    pub fn new() -> Self {
        HeroDto {
            id: i32::default(),
            name: String::default(),
        }
    }

    pub fn new_data(id: i32, name: String) -> HeroDto {
        HeroDto {
            id,
            name,
        }
    }
}

impl PartialEq for HeroDto {
    fn eq(&self, other: &Self) -> bool {
        &self.name == &other.name
    }

    fn ne(&self, other: &Self) -> bool {
        &self.name != &other.name
    }
}

impl Eq for HeroDto {

}

impl From<&HeroDto> for HeroDto {
    fn from(dto: &HeroDto) -> Self {
        HeroDto {
            id: dto.id,
            name: String::from(&dto.name),
        }
    }
}

pub struct Hero {
    pub state: HeroDto,
    pub original: HeroDto,
}

impl Hero {
    pub fn new(dto: HeroDto) -> Self {
        Hero {
            state: HeroDto::from(&dto),
            original: HeroDto::from(&dto),
        }
    }

    pub fn create() -> Self {
        Hero {
            state: HeroDto::new(),
            original: HeroDto::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.state.name
    }

    pub fn assign_name(&mut self, name: &str) -> Result<()> {
        let state = &mut self.state;

        if &state.name == name {
            return Ok(())
        }

        if &state.name == &String::default() {
            state.name = String::from(name);
            Ok(())
        } else {
            Err(AlreadyAssignedError::boxed("name"))
        }
    }

    pub fn dto(&self) -> HeroDto {
        HeroDto::from(&self.state)
    }

    pub fn apply_dto(hero: &mut Hero, dto: &HeroDto) -> Result<()> {
        hero.assign_name(&dto.name)?;

        Ok(())
    }

    pub fn is_modified(&self) -> bool {
        &self.state != &self.original
    }
}

pub type HeroResult = Result<Hero>;
pub type HeroOptionResult = Result<std::option::Option<Hero>>;
pub mod repository;
mod stats;
pub mod errors;

use serde::{Serialize, Deserialize};
use serde::export::PhantomData;
use super::Result;
use errors::AlreadyAssignedError;

#[derive(Serialize, Deserialize, Clone)]
pub enum HeroClass {
    None,
    Archer,
    Warrior,
    Wizard,
}

impl From<String> for HeroClass {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Archer" => HeroClass::Archer,
            "Warrior" => HeroClass::Warrior,
            "Wizard" => HeroClass::Wizard,
            _ => HeroClass::None
        }
    }
}

impl From<&HeroClass> for String {
    fn from(c: &HeroClass) -> Self {
        match c {
            &HeroClass::Archer => String::from("Archer"),
            &HeroClass::Warrior => String::from("Warrior"),
            &HeroClass::Wizard => String::from("Wizard"),
            &HeroClass::None => String::from("None")
        }
    }
}



impl Default for HeroClass {
    fn default() -> Self {
        HeroClass::None
    }
}

#[derive(Serialize, Deserialize)]
pub struct HeroDto {
    pub id: i32,
    pub name: String,
    pub class: HeroClass
}

impl HeroDto {
    pub fn new() -> Self {
        HeroDto {
            id: i32::default(),
            name: String::default(),
            class: HeroClass::default(),
        }
    }

    pub fn new_data(id: i32, name: String, class: HeroClass) -> HeroDto {
        HeroDto {
            id,
            name,
            class
        }
    }
}

impl From<&HeroDto> for HeroDto {
    fn from(dto: &HeroDto) -> Self {
        HeroDto { id: dto.id,
            name: String::from(&dto.name),
            class:  dto.class.clone()}
    }
}

pub struct Hero<'a> {
    pub state: HeroDto,
    pub original: HeroDto,
    unused: std::marker::PhantomData<&'a ()>,
}

impl<'a> Hero<'a> {
    pub fn new(dto: HeroDto) -> Self {
        Hero {
            state: HeroDto::from(&dto),
            original: HeroDto::from(&dto),
            unused: PhantomData::default()
        }
    }

    pub fn create() -> Self {
        Hero {
            state: HeroDto::new(),
            original: HeroDto::new(),
            unused: PhantomData::default()
        }
    }

    pub fn name(&'a self) -> &'a str {
        &self.state.name
    }

    pub fn assign_name(&mut self, name: &str) -> Result<()> {
        let state = &mut self.state;

        if &state.name == &String::default() {
            state.name = String::from(name);
            Ok(())
        } else {
            Err(AlreadyAssignedError::boxed("name"))
        }
    }

    pub fn class(&'a self) -> &'a HeroClass { &self.state.class }

    pub fn assign_class(&mut self, class: &HeroClass) -> Result<()> {
        let state = &mut self.state;

        let is_assigned = match state.class {
            HeroClass::None => false,
            _ => true
        };

        if !is_assigned  {
            state.class = class.clone();
            Ok(())
        } else {
            Err(AlreadyAssignedError::boxed("class"))
        }
    }

    pub fn dto(&self) -> HeroDto {
        HeroDto::from(&self.state)
    }

    pub fn apply_dto(hero: &mut Hero<'a>, dto: &HeroDto) -> Result<()> {
        hero.assign_name(&dto.name)?;
        hero.assign_class(&dto.class)?;

        Ok(())
    }
}

pub type HeroResult<'a> = Result<Hero<'a>>;
pub type HeroOptionResult<'a> = Result<std::option::Option<Hero<'a>>>;
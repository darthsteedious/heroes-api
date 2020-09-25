use serde::{Serialize, Deserialize};
use serde::export::PhantomData;
use super::Result;

pub mod repository;
mod stats;

#[derive(Serialize, Deserialize, Clone)]
pub enum HeroClass {
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
            _ => panic!("Invalid hero class type")
        }
    }
}

impl From<&HeroClass> for String {
    fn from(c: &HeroClass) -> Self {
        match c {
            &HeroClass::Archer => String::from("Archer"),
            &HeroClass::Warrior => String::from("Warrior"),
            &HeroClass::Wizard => String::from("Wizard")
        }
    }
}



impl Default for HeroClass {
    fn default() -> Self {
        HeroClass::Warrior
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

    pub fn name(&'a self) -> &'a str {
        &self.state.name
    }

    pub fn class(&'a self) -> &'a HeroClass { &self.state.class }

    pub fn dto(&'a self) -> HeroDto {
        HeroDto::from(&self.state)
    }
}

pub type HeroResult<'a> = Result<Hero<'a>>;
pub type HeroOptionResult<'a> = Result<std::option::Option<Hero<'a>>>;
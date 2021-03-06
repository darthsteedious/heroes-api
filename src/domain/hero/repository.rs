use async_trait::async_trait;
use crate::domain::Result;
use super::{Hero, HeroOptionResult};

#[async_trait]
pub trait HeroRepository {
    async fn get_hero<'a>(&'_ self, id: i32) -> HeroOptionResult<'a>;
    async fn save_hero<'a>(&'_ self, hero: &'a Hero<'a>) -> Result<i32>;
}
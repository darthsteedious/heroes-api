use async_trait::async_trait;
use crate::domain::Result;
use super::{Hero, HeroOptionResult};

#[async_trait]
pub trait HeroRepository {
    async fn get_hero(&self, id: i32) -> HeroOptionResult;
    async fn save_hero(&self, hero: &Hero) -> Result<i32>;
}
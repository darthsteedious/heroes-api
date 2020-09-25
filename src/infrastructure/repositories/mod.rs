use crate::domain::hero::repository::HeroRepository;
use deadpool_postgres::{Pool};
use crate::infrastructure::repositories::postgres_hero_repository::PostgresHeroRepository;

mod postgres_hero_repository;


pub fn create_hero_repository(pool: Pool) -> impl HeroRepository {
    PostgresHeroRepository::new(pool)
}
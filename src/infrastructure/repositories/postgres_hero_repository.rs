use async_trait::async_trait;
use crate::domain::hero::{Hero, HeroDto, repository::HeroRepository, HeroOptionResult};
use deadpool_postgres::{Pool, Client};
use crate::domain::Result;
use serde::export::Formatter;

#[derive(Debug)]
pub struct NoRowError;

impl std::fmt::Display for NoRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No rows returned")
    }
}

impl std::error::Error for NoRowError {}

pub struct PostgresHeroRepository {
    pg_pool: Pool
}

impl PostgresHeroRepository {
    pub fn new(db_client: Pool) -> impl HeroRepository {
        PostgresHeroRepository { pg_pool: db_client }
    }

    fn get_pool(&self) -> &Pool {
        &self.pg_pool
    }

    async fn get_client(&self) -> crate::domain::Result<Client> {
        let pool = self.get_pool();


        let result: Client = pool.get().await?;

        Ok(result)
    }
}

#[async_trait]
impl HeroRepository for PostgresHeroRepository {
    async fn get_hero(&self, id: i32) -> HeroOptionResult {
        let client = self.get_client().await?;
        let q = "
        select h.hero_id,
            h.name
        from hero h
        where h.hero_id = $1";
        let result = client
            .query_one(q, &[ &id ])
            .await
            .map_or_else(|e| {
                if format!("{}", e) == "query returned an unexpected number of rows" {
                    Ok(None)
                } else {
                    Err(Box::new(e))
                }
            }, |v| Ok(Some(v)))?;

        match result {
            Some(row) => {
                let dto = HeroDto::new_data(row.get(0), row.get(1));

                Ok(Some(Hero::new(dto)))
            }
            None => Ok(None)
        }
    }

    async fn save_hero(&self, hero: &Hero) -> Result<i32> {
        if !hero.is_modified() {
            let state = &hero.state;

            return Ok(state.id)
        }

        let client = self.get_client().await?;

        let q = "
        insert into hero(name)
        values ($1)
        on conflict (name)
        do nothing
        returning hero_id";

        let result = client.query_one(q,
                                      &[
                                          &hero.name()
                                      ]).await?;

        let id: i32 = result.try_get(0)?;
        Ok(id)
    }
}
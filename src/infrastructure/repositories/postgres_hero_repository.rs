use async_trait::async_trait;
use crate::domain::hero::{Hero, HeroDto, repository::HeroRepository, HeroOptionResult};
use deadpool_postgres::{Pool, Client};
use crate::domain::Result;
use tokio_postgres::error::SqlState;
use serde::export::Formatter;

#[derive(Debug)]
pub struct NoRowError;

impl std::fmt::Display for NoRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No rows returned")
    }
}

impl std::error::Error for NoRowError {}

impl NoRowError {
    pub fn boxed() -> Box<dyn std::error::Error> {
        Box::new(NoRowError)
    }
}

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
    async fn get_hero<'a>(&self, id: i32) -> HeroOptionResult<'a> {
        let client = self.get_client().await?;
        let q = "
        select h.hero_id,
            h.name,
            hc.name as class
        from hero h
        join hero_class hc on h.hero_class_id = hc.hero_class_id
        where h.hero_id = $1";
        let params = &[&id];
        let result = client
            .query_one(q, params)
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
                let class: String = row.get(2);

                let dto = HeroDto::new_data(row.get(0), row.get(1), class.into());

                Ok(Some(Hero::new(dto)))
            }
            None => Ok(None)
        }
    }

    async fn save_hero<'a>(&self, hero: &'a Hero<'a>) -> Result<i32> {
        let client = self.get_client().await?;

        let class: String = hero.class().into();

        let class_id: i32 = client.query_one("select hero_class_id from hero_class where name = $1",
                                             &[&class])
            .await?
            .get(0);

        let q = "
        insert into hero(name, hero_class_id)
        values ($1, $2)
        on conflict (name)
        do nothing
        returning hero_id";

        // let class: String = hero.class().into();

        let result = client.query_one(q,
                                      &[
                                          &hero.name(),
                                          &class_id
                                      ]).await?;

        let id: i32 = result.try_get(0)?;
        Ok(id)
    }
}
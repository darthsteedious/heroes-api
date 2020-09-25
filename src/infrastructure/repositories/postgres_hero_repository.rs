use async_trait::async_trait;
use crate::domain::hero::{Hero, HeroDto, repository::HeroRepository, HeroOptionResult};
use deadpool_postgres::{Pool, Client};
use crate::domain::ResultOption;

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
        select hero_id,
            name
        from hero h
        where h.hero_id = $1";
        let result = client.query_one(q,
                                      &[
                                          &id
                                      ]).await?;

        let class: String = result.get(2);

        let dto = HeroDto::new_data(result.get(0), result.get(1), class.into());

        Ok(Some(Hero::new(dto)))
    }

    async fn save_hero<'a>(&self, hero: &'a Hero<'a>) -> ResultOption<i32> {
        let client = self.get_client().await?;

        let q = "
        insert into hero(name, class)
        values ($1, $2)
        returning hero_id";

        let class: String = hero.class().into();

        let result = client.query_one(q,
                                      &[
                                          &hero.name(),
                                          &class
                                      ]).await?;

        let id: i32 = result.try_get(0)?;
        Ok(Some(id))
    }
}
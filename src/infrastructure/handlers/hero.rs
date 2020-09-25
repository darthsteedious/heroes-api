use actix_web::{web, Scope, get, post, HttpResponse, Result};
use deadpool_postgres::Pool;
use serde::Deserialize;
use crate::domain::hero::repository::HeroRepository;
use crate::infrastructure::repositories::create_hero_repository;
use crate::domain::hero::{HeroDto, Hero};

#[derive(Debug, Deserialize)]
struct GetHero {
    hero_id: i32
}

#[get("/{hero_id}")]
async fn get_hero(req: web::Path<GetHero>, d: web::Data<Pool>) -> Result<HttpResponse> {
    let repo = create_hero_repository(d.get_ref().clone());

    match repo.get_hero(req.hero_id).await {
        Ok(result) => match result {
            Some(hero) => Ok(HttpResponse::Ok().json(hero.dto())),
            None => Ok(HttpResponse::NotFound().finish())
        },
        Err(e) => {
            eprintln!("Error getting user: {}", e);

            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[post("")]
async fn save_hero(req: web::Json<HeroDto>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let repo = create_hero_repository(pool.get_ref().clone());

    let hero = &Hero::new(req.into_inner());

    match repo.save_hero(hero).await {
        Ok(result) => match result {
            Some(id) => Ok(HttpResponse::Created().body(format!("{}", id))),
            None => Ok(HttpResponse::InternalServerError().body("This is the none block and it should die"))
        },
        Err(e) => {
            eprintln!("Error saving user: {}", e);

            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub fn register_hero_handlers(scope: Scope) -> Scope {
    scope
        .service(get_hero)
        .service(save_hero)
}
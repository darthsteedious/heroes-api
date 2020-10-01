use actix_web::{web, Scope, get, post, put, HttpResponse, Result, error};
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

    repo.get_hero(req.hero_id)
        .await
        .map_err(|e| {
            eprintln!("Error getting user: {}", e);

            error::ErrorInternalServerError(e)
        })
        .map(|h| match h {
            Some(hero) => HttpResponse::Ok().json(hero.dto()),
            None => HttpResponse::NotFound().finish()
        })
}

#[post("")]
async fn create_hero(req: web::Json<HeroDto>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let repo = create_hero_repository(pool.get_ref().clone());

    let hero = &mut Hero::create();

    Hero::apply_dto(hero, &req.into_inner())
        .map_err(|e| error::ErrorBadRequest(e))?;

    repo.save_hero(hero)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))
        .map(|id| HttpResponse::Created().body(format!("{}", id)))
}

#[put("/{hero_id}")]
async fn update_hero(dto: web::Json<HeroDto>, id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let repo = create_hero_repository(pool.get_ref().clone());

    let hero_option = repo.get_hero(id.0)
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    match hero_option {
        Some(mut hero) => {
            Hero::apply_dto(&mut hero, &dto.into_inner())
                .map_err(|e| error::ErrorBadRequest(e))?;

            repo.save_hero(&hero)
                .await
                .map_err(|e| error::ErrorInternalServerError(e))
                .map(|_| HttpResponse::NoContent().finish())
        }
        None => Ok(HttpResponse::NotFound().finish())
    }
}

pub fn register_hero_handlers(scope: Scope) -> Scope {
    scope
        .service(get_hero)
        .service(create_hero)
        .service(update_hero)
}
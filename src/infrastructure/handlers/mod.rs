use actix_web::{web, Scope};

mod hero;

use hero::register_hero_handlers;

pub fn register_handlers(scope: Scope) -> Scope {
    scope
        .service(register_hero_handlers(web::scope("/heroes")))
}
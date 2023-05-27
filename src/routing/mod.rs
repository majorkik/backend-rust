use self::characters::{
    character, characters, create_character, remove_character, update_character,
};
use self::index::index_route;

mod characters;
pub mod index;

use actix_web::web;

pub fn config_service(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index_route)).service(
        web::scope("").service(
            web::scope("/characters")
                .service(
                    web::resource("")
                        .route(web::get().to(characters))
                        .route(web::post().to(create_character)),
                )
                .service(
                    web::scope("/{character_id}").service(
                        web::resource("")
                            .route(web::get().to(character))
                            .route(web::put().to(update_character))
                            .route(web::delete().to(remove_character)),
                    ),
                ),
        ),
    );
}

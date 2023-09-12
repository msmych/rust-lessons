use std::sync::Arc;

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

use crate::{config::AppConfig, services::Services};

use self::{
    account_routing::{create_account, get_account},
    ingredient_routing::{create_ingredient, get_ingredient},
    menu_routing::{create_menu, get_menu},
    recipe_routing::{create_recipe, get_recipe},
};

pub mod account_routing;
pub mod ingredient_routing;
pub mod menu_routing;
pub mod recipe_routing;

pub async fn setup_server(config: Arc<AppConfig>, services: Arc<Services>) -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .app_data(services.account_service.clone())
            .app_data(services.menu_service.clone())
            .app_data(services.ingredient_service.clone())
            .app_data(services.recipe_service.clone())
            .service(create_account)
            .service(get_account)
            .service(create_menu)
            .service(get_menu)
            .service(create_ingredient)
            .service(get_ingredient)
            .service(create_recipe)
            .service(get_recipe)
            .wrap(Logger::default())
    })
    .bind((config.host.clone(), config.port))?
    .run()
    .await
}

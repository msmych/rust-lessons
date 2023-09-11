use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use app::routing::{
    account_routing::create_account, account_routing::get_account,
    ingredient_routing::create_ingredient, ingredient_routing::get_ingredient,
    menu_routing::create_menu, menu_routing::get_menu, recipe_routing::create_recipe,
    recipe_routing::get_recipe,
};
use cocktails::domain::account::AccountService;
use cocktails::domain::ingredient::IngredientService;
use cocktails::domain::menu::MenuService;
use cocktails::domain::recipe::RecipeService;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: AppConfig =
        confy::load_path("config/app-local.yaml").expect("Failed to load config");
    let db = Arc::new(setup_db(config.db).await);
    let account_service = web::Data::new(AccountService::create(Arc::clone(&db)));
    let menu_service = web::Data::new(MenuService::create(Arc::clone(&db)));
    let ingredient_service = web::Data::new(IngredientService::create(Arc::clone(&db)));
    let recipe_service = web::Data::new(RecipeService::create(Arc::clone(&db)));
    HttpServer::new(move || {
        App::new()
            .app_data(account_service.clone())
            .app_data(menu_service.clone())
            .app_data(ingredient_service.clone())
            .app_data(recipe_service.clone())
            .service(create_account)
            .service(get_account)
            .service(create_menu)
            .service(get_menu)
            .service(create_ingredient)
            .service(get_ingredient)
            .service(create_recipe)
            .service(get_recipe)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}

async fn setup_db(config: DbConfig) -> Surreal<Client> {
    let db = Surreal::new::<Ws>(config.address)
        .await
        .expect("Failed to connect to DB");
    db.signin(Root {
        username: &config.username,
        password: &config.password,
    })
    .await
    .expect("Failed to sign in to DB");
    db.use_ns(config.ns)
        .use_db(config.db)
        .await
        .expect("Failed to select namespace or DB");
    db
}

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    host: String,
    port: u16,
    db: DbConfig,
}

#[derive(Serialize, Deserialize, Default)]
struct DbConfig {
    address: String,
    username: String,
    password: String,
    ns: String,
    db: String,
}

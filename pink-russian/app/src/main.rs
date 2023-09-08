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
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(setup_db().await);
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
    .bind(("localhost", 8080))?
    .run()
    .await
}

async fn setup_db() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("msg");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("msg");
    db.use_ns("test").use_db("test").await.expect("msg");
    db
}

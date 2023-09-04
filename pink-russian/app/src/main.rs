use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use app::{
    create_account, create_ingredient, create_menu, create_recipe, get_account, get_ingredient,
    get_menu, get_recipe,
};
use cocktails::menu::ingredient::IngredientService;
use cocktails::menu::recipe::RecipeService;
use cocktails::menu::MenuService;
use cocktails::AccountService;
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

use actix_web::{web, App, HttpServer};
use app::{create_account, create_ingredient, create_menu, create_recipe};
use cocktails::menu::ingredient::IngredientService;
use cocktails::menu::recipe::RecipeService;
use cocktails::menu::MenuService;
use cocktails::AccountService;

use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Mutex::new(AccountService::create())))
            .app_data(web::Data::new(Mutex::new(MenuService::create())))
            .app_data(web::Data::new(Mutex::new(IngredientService::create())))
            .app_data(web::Data::new(Mutex::new(RecipeService::create())))
            .service(create_account)
            .service(create_menu)
            .service(create_ingredient)
            .service(create_recipe)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

use actix_web::{web, App, HttpServer};
use app::{create_account, create_ingredient, create_menu, create_recipe};
use cocktails::menu::ingredient::IngredientService;
use cocktails::menu::recipe::RecipeService;
use cocktails::menu::MenuService;
use cocktails::AccountService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let account_service = AccountService::create();
        let menu_service = MenuService::create();
        let ingredient_service = IngredientService::create();
        let recipe_service = RecipeService::create();
        App::new()
            .app_data(web::Data::new(account_service))
            .app_data(web::Data::new(menu_service))
            .app_data(web::Data::new(ingredient_service))
            .app_data(web::Data::new(recipe_service))
            .service(create_account)
            .service(create_menu)
            .service(create_ingredient)
            .service(create_recipe)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

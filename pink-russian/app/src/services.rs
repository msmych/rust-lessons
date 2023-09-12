use std::sync::Arc;

use actix_web::web::Data;
use cocktails::domain::{
    account::AccountService, ingredient::IngredientService, menu::MenuService,
    recipe::RecipeService,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub struct Services {
    pub account_service: Data<AccountService>,
    pub menu_service: Data<MenuService>,
    pub ingredient_service: Data<IngredientService>,
    pub recipe_service: Data<RecipeService>,
}

impl Services {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        Self {
            account_service: Data::new(AccountService::create(db.clone())),
            menu_service: Data::new(MenuService::create(db.clone())),
            ingredient_service: Data::new(IngredientService::create(db.clone())),
            recipe_service: Data::new(RecipeService::create(db.clone())),
        }
    }
}

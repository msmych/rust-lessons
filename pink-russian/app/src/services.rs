use std::sync::Arc;

use actix_web::web::Data;
use cocktails::domain::{
    account::AccountService, ingredient::IngredientService,
    cocktail::CocktailService,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub struct Services {
    pub account_service: Data<AccountService>,
    pub ingredient_service: Data<IngredientService>,
    pub cocktail_service: Data<CocktailService>,
}

impl Services {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        Self {
            account_service: Data::new(AccountService::create(db.clone())),
            ingredient_service: Data::new(IngredientService::create(db.clone())),
            cocktail_service: Data::new(CocktailService::create(db.clone())),
        }
    }
}

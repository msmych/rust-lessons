use std::{collections::HashMap, sync::Mutex};

use actix_web::{post, web, HttpResponse, Responder};
use cocktails::{
    menu::{
        ingredient::{Amount, Ingredient, IngredientService},
        recipe::{Recipe, RecipeService},
        Menu, MenuService,
    },
    Account, AccountService,
};
use serde::Deserialize;
use uuid::Uuid;

#[post("/accounts")]
pub async fn create_account(account_service: web::Data<Mutex<AccountService>>) -> impl Responder {
    let account = Account::new();
    account_service.lock().unwrap().add(account.clone());
    HttpResponse::Ok().body(account.id().to_string())
}

#[derive(Deserialize)]
pub struct CreateMenuRequest {
    #[serde(rename = "accountId")]
    account_id: Uuid,
    name: String,
}

#[post("/menus")]
pub async fn create_menu(
    rq: web::Json<CreateMenuRequest>,
    menu_service: web::Data<Mutex<MenuService>>,
) -> impl Responder {
    let rq = rq.into_inner();
    let menu = Menu::new(rq.account_id, &rq.name);
    menu_service.lock().unwrap().add(menu.clone());
    HttpResponse::Ok().body(menu.id().to_string())
}

#[derive(Deserialize)]
pub struct CreateIngredientRequest {
    name: String,
    #[serde(rename = "ownerId")]
    owner_id: Option<Uuid>,
}

#[post("/ingredients")]
pub async fn create_ingredient(
    rq: web::Json<CreateIngredientRequest>,
    ingredient_service: web::Data<Mutex<IngredientService>>,
) -> impl Responder {
    let rq = rq.into_inner();
    let ingredient = Ingredient::new(&rq.name, rq.owner_id);
    ingredient_service.lock().unwrap().add(ingredient.clone());
    HttpResponse::Ok().body(ingredient.id().to_string())
}

#[derive(Deserialize)]
pub struct RecipeIngredient {
    #[serde(rename = "ingredientId")]
    ingredient_id: Uuid,
    amount: Option<u8>,
    unit: String,
}

#[derive(Deserialize)]
pub struct CreateRecipeRequest {
    name: String,
    ingredients: Vec<RecipeIngredient>,
    instruction: String,
}

#[post("/recipes")]
pub async fn create_recipe(
    rq: web::Json<CreateRecipeRequest>,
    recipe_service: web::Data<Mutex<RecipeService>>,
) -> impl Responder {
    let rq = rq.into_inner();
    let recipe = Recipe::new(
        &rq.name,
        rq.ingredients.iter().fold(HashMap::new(), |mut acc, i| {
            acc.insert(
                i.ingredient_id,
                match i.unit.as_str() {
                    "Oz" => Amount::Oz(i.amount.unwrap()),
                    "Cl" => Amount::Cl(i.amount.unwrap()),
                    _ => Amount::Some,
                },
            );
            acc
        }),
        &rq.instruction,
    );
    recipe_service.lock().unwrap().add(recipe.clone());
    HttpResponse::Ok().body(recipe.id().to_string())
}

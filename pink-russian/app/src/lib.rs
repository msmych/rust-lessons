use std::collections::HashMap;

use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use cocktails::{
    menu::{
        ingredient::{Amount, Ingredient, IngredientService},
        recipe::{Recipe, RecipeService},
        Menu, MenuService,
    },
    Account, AccountService,
};
use serde::Deserialize;

#[post("/accounts")]
pub async fn create_account(
    rq: Json<CreateAccountRequest>,
    account_service: web::Data<AccountService>,
) -> impl Responder {
    let account = Account::new(rq.into_inner().name.as_str());
    let record_id = account_service.add(account).await.expect("msg");
    HttpResponse::Created().json(record_id)
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    name: String,
}

#[get("/accounts/{id}")]
pub async fn get_account(
    path: web::Path<String>,
    account_service: web::Data<AccountService>,
) -> Json<Account> {
    let account = account_service
        .get(path.into_inner())
        .await
        .expect("msg")
        .expect("msg");
    Json(account)
}

#[derive(Deserialize)]
pub struct CreateMenuRequest {
    #[serde(rename = "accountId")]
    account_id: String,
    name: String,
}

#[post("/menus")]
pub async fn create_menu(
    rq: web::Json<CreateMenuRequest>,
    menu_service: web::Data<MenuService>,
) -> impl Responder {
    let rq = rq.into_inner();
    let menu = Menu::new(rq.account_id, &rq.name);
    let record_id = menu_service.add(menu.clone()).await.expect("msg");
    HttpResponse::Created().json(record_id)
}

#[get("/menus/{id}")]
pub async fn get_menu(path: web::Path<String>, menu_service: web::Data<MenuService>) -> Json<Menu> {
    let menu = menu_service
        .get(path.into_inner())
        .await
        .expect("msg")
        .expect("msg");
    Json(menu)
}

#[derive(Deserialize)]
pub struct CreateIngredientRequest {
    name: String,
    #[serde(rename = "ownerId")]
    owner_id: Option<String>,
}

#[post("/ingredients")]
pub async fn create_ingredient(
    rq: web::Json<CreateIngredientRequest>,
    ingredient_service: web::Data<IngredientService>,
) -> impl Responder {
    let rq = rq.into_inner();
    let record_id = ingredient_service
        .add(Ingredient::new(&rq.name, rq.owner_id))
        .await
        .expect("msg");
    HttpResponse::Created().json(record_id)
}

#[get("/ingredients/{id}")]
pub async fn get_ingredient(
    path: web::Path<String>,
    ingredient_service: web::Data<IngredientService>,
) -> Json<Ingredient> {
    let ingredient = ingredient_service
        .get(path.into_inner())
        .await
        .expect("msg")
        .expect("msg");
    Json(ingredient)
}

#[derive(Deserialize)]
pub struct RecipeIngredient {
    #[serde(rename = "ingredientId")]
    ingredient_id: String,
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
    recipe_service: web::Data<RecipeService>,
) -> impl Responder {
    let rq = rq.into_inner();
    let record_id = recipe_service
        .add(Recipe::new(
            &rq.name,
            rq.ingredients.iter().fold(HashMap::new(), |mut acc, i| {
                acc.insert(
                    i.ingredient_id.clone(),
                    match i.unit.as_str() {
                        "Oz" => Amount::Oz(i.amount.unwrap()),
                        "Cl" => Amount::Cl(i.amount.unwrap()),
                        _ => Amount::Some,
                    },
                );
                acc
            }),
            &rq.instruction,
        ))
        .await
        .expect("msg");
    HttpResponse::Created().json(record_id)
}

#[get("/recipes/{id}")]
pub async fn get_recipe(
    path: web::Path<String>,
    recipe_service: web::Data<RecipeService>,
) -> Json<Recipe> {
    let recipe = recipe_service
        .get(path.into_inner())
        .await
        .expect("msg")
        .expect("msg");
    Json(recipe)
}

use std::collections::HashMap;

use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use cocktails::domain::{
    ingredient::Amount,
    recipe::{Recipe, RecipeService},
};
use serde::Deserialize;

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
        .await;
    HttpResponse::Created().json(record_id)
}

#[get("/recipes/{id}")]
pub async fn get_recipe(
    path: web::Path<String>,
    recipe_service: web::Data<RecipeService>,
) -> Json<Recipe> {
    let recipe = recipe_service.get(path.into_inner()).await;
    Json(recipe)
}

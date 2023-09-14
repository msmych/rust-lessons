use std::collections::HashMap;

use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use cocktails::domain::{
    ingredient::Amount,
    cocktail::{Cocktail, CocktailService},
};
use serde::Deserialize;

#[post("/cocktails")]
pub async fn create_cocktail(
    rq: Json<CreateCocktailRequest>,
    cocktail_service: web::Data<CocktailService>,
) -> impl Responder {
    let rq = rq.into_inner();
    let record_id = cocktail_service.add(rq.to_cocktail()).await;
    HttpResponse::Created().json(record_id)
}

#[get("/cocktails/{id}")]
pub async fn get_cocktail(
    path: web::Path<String>,
    cocktail_service: web::Data<CocktailService>,
) -> Json<Cocktail> {
    let cocktail = cocktail_service.get(path.into_inner()).await;
    Json(cocktail)
}

#[derive(Deserialize)]
pub struct CreateCocktailRequest {
    name: String,
    #[serde(rename = "accountId")]
    account_id: String,
    ingredients: Vec<CocktailIngredient>,
    recipe: Vec<String>,
}

impl CreateCocktailRequest {
    fn to_cocktail(&self) -> Cocktail {
        Cocktail::new(
            &self.name,
            &self.account_id,
            self.ingredients.iter().fold(HashMap::new(), |mut acc, i| {
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
            self.recipe.clone(),
        )
    }
}

#[derive(Deserialize)]
pub struct CocktailIngredient {
    #[serde(rename = "ingredientId")]
    ingredient_id: String,
    amount: Option<u8>,
    unit: String,
}

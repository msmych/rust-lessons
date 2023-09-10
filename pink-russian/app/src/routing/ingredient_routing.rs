use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use cocktails::domain::ingredient::{Ingredient, IngredientService};
use serde::Deserialize;

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
        .await;
    HttpResponse::Created().json(record_id)
}

#[get("/ingredients/{id}")]
pub async fn get_ingredient(
    path: web::Path<String>,
    ingredient_service: web::Data<IngredientService>,
) -> Json<Ingredient> {
    let ingredient = ingredient_service.get(path.into_inner()).await;
    Json(ingredient)
}

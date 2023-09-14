use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use cocktails::domain::ingredient::{Ingredient, IngredientService};
use serde::Deserialize;

#[post("/ingredients")]
pub async fn create_ingredient(
    rq: Json<CreateIngredientRequest>,
    ingredient_service: web::Data<IngredientService>,
) -> impl Responder {
    let rq = rq.into_inner();
    let record_id = ingredient_service.add(rq.to_ingredient()).await;
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

#[derive(Deserialize)]
pub struct CreateIngredientRequest {
    id: Option<String>,
    name: String,
    #[serde(rename = "accountId")]
    account_id: Option<String>,
}

impl CreateIngredientRequest {
    fn to_ingredient(&self) -> Ingredient {
        if let Some(id) = self.id.as_ref() {
            Ingredient::default(id, &self.name)
        } else {
            Ingredient::new(&self.name, self.account_id.as_ref().unwrap())
        }
    }
}

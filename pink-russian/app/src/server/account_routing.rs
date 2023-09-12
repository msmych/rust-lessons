use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use cocktails::domain::account::{Account, AccountService};
use serde::Deserialize;

#[post("/accounts")]
pub async fn create_account(
    rq: Json<CreateAccountRequest>,
    account_service: web::Data<AccountService>,
) -> impl Responder {
    let account = Account::new(rq.into_inner().name.as_str());
    let record_id = account_service.add(account).await;
    HttpResponse::Created().json(record_id)
}

#[get("/accounts/{id}")]
pub async fn get_account(
    path: web::Path<String>,
    account_service: web::Data<AccountService>,
) -> Json<Account> {
    let account = account_service.get(path.into_inner()).await;
    Json(account)
}

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    name: String,
}

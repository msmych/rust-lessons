use std::sync::Mutex;

use actix_web::{post, web, HttpResponse, Responder};
use cocktails::{Account, AccountService};

pub async fn ciao() -> impl Responder {
    HttpResponse::Ok().body("Ciao")
}

#[post("/accounts")]
pub async fn create_account(account_service: web::Data<Mutex<AccountService>>) -> impl Responder {
    let account = Account::new();
    account_service.lock().unwrap().add(account.clone());
    HttpResponse::Ok().body(account.id().to_string())
}

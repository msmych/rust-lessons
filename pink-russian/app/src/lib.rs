use std::sync::Mutex;

use actix_web::{post, web, HttpResponse, Responder};
use cocktails::{
    menu::{Menu, MenuService},
    Account, AccountService,
};
use serde::Deserialize;
use uuid::Uuid;

pub async fn ciao() -> impl Responder {
    HttpResponse::Ok().body("Ciao")
}

#[post("/accounts")]
pub async fn create_account(account_service: web::Data<Mutex<AccountService>>) -> impl Responder {
    let account = Account::new();
    account_service.lock().unwrap().add(account.clone());
    HttpResponse::Ok().body(account.id().to_string())
}

#[derive(Debug, Deserialize)]
pub struct CreateMenuRequest {
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

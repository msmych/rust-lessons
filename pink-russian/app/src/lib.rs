use std::sync::Mutex;

use actix_web::{post, web, HttpResponse, Responder};
use cocktails::{
    menu::{Menu, MenuService},
    Account, AccountService,
};
use uuid::Uuid;

pub struct Services {
    pub account_service: AccountService,
    pub menu_service: MenuService,
}

pub async fn ciao() -> impl Responder {
    HttpResponse::Ok().body("Ciao")
}

#[post("/accounts")]
pub async fn create_account(services: web::Data<Mutex<Services>>) -> impl Responder {
    let account = Account::new();
    services
        .lock()
        .unwrap()
        .account_service
        .add(account.clone());
    HttpResponse::Ok().body(account.id().to_string())
}

pub async fn create_menu(
    account_id: String,
    services: web::Data<Mutex<Services>>,
) -> impl Responder {
    let menu = Menu::new(Uuid::from(account_id), "Summer menu");
    services.lock().unwrap().menu_service.add(menu);
    HttpResponse::Ok().body(menu.id().to_string())
}

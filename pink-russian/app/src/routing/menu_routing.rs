use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use cocktails::domain::menu::{Menu, MenuService};
use serde::Deserialize;

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
    let record_id = menu_service.add(menu).await;
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

use crate::data::{write_data, AppData, Transaction};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/data", web::get().to(get_all_data))
        .route("/transactions", web::post().to(add_transaction))
        .route("/transactions/{index}", web::put().to(update_transaction))
        .route(
            "/transactions/{index}",
            web::delete().to(delete_transaction),
        );
}

async fn get_all_data(data: web::Data<Mutex<AppData>>) -> impl Responder {
    let data = data.lock().unwrap();
    HttpResponse::Ok().json(&*data)
}

async fn add_transaction(
    data: web::Data<Mutex<AppData>>,
    new_transaction: web::Json<Transaction>,
) -> impl Responder {
    let mut data = data.lock().unwrap();
    data.transactions.push(new_transaction.into_inner());
    write_data(&*data);
    HttpResponse::Ok().body("Transaction added")
}

async fn update_transaction(
    data: web::Data<Mutex<AppData>>,
    path: web::Path<usize>,
    updated_transaction: web::Json<Transaction>,
) -> impl Responder {
    let mut data = data.lock().unwrap();
    let index = path.into_inner();
    if index >= data.transactions.len() {
        return HttpResponse::BadRequest().body("Invalid index");
    }
    data.transactions[index] = updated_transaction.into_inner();
    write_data(&*data);
    HttpResponse::Ok().body("Transaction updated")
}

async fn delete_transaction(
    data: web::Data<Mutex<AppData>>,
    path: web::Path<usize>,
) -> impl Responder {
    let mut data = data.lock().unwrap();
    let index = path.into_inner();
    if index >= data.transactions.len() {
        return HttpResponse::BadRequest().body("Invalid index");
    }
    data.transactions.remove(index);
    write_data(&*data);
    HttpResponse::Ok().body("Transaction deleted")
}

use actix_web::web;

mod user_controller;
pub mod user_repository;
pub mod dto;
pub mod models;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(user_controller::get_all_users)
        .service(user_controller::create_user)
        .service(user_controller::get_user_by_id)
        .service(user_controller::update_user_by_id)
        .service(user_controller::delete_user_by_id);
}
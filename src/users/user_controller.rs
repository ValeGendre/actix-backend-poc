use actix_web::{delete, error, get, patch, post, web, HttpResponse, Responder};

use crate::users::dto::create_user_input::CreateUserInput;
use crate::users::models::new_user::NewUser;

use crate::users::dto::update_user_input::UpdateUserInput;
use crate::users::models::update_user::UpdateUser;

use crate::users::user_repository::{find_all, create, find_one, delete_one, update_one};
use crate::DbPool;

#[post("/users")]
pub async fn create_user(pool: web::Data<DbPool>, form: web::Json<CreateUserInput>) -> actix_web::Result<impl Responder> {
    let new_saved_user = web::block(move || {
        let mut connection = pool.get()?;
        let new_user = NewUser {
            firstname: &form.firstname,
            lastname: &form.lastname,
            email: &form.email,
            administrator: &form.administrator
        };
        create(&mut connection, &new_user)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(new_saved_user))
}

#[get("/users")]
pub async fn get_all_users(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let user_list = web::block(move || {
        let mut connection = pool.get()?;
        return find_all(&mut connection);
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user_list))
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let user_id = user_id.into_inner();
    let user = web::block(move || {
        let mut connection = pool.get()?;
        return find_one(&mut connection, user_id);
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[patch("/users/{user_id}")]
pub async fn update_user_by_id(pool: web::Data<DbPool>, user_id: web::Path<i32>, form: web::Json<UpdateUserInput>) -> actix_web::Result<impl Responder> {
    let user_id = user_id.into_inner();
    let user = web::block(move || {
        let mut connection = pool.get()?;
        let update_user = UpdateUser {
            firstname: form.firstname.as_deref(),
            lastname: form.lastname.as_deref(),
            email: form.email.as_deref(),
            administrator: form.administrator.as_ref()
        };
        return update_one(&mut connection, user_id, &update_user);
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{user_id}")]
pub async fn delete_user_by_id(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let user_id = user_id.into_inner();
    let user = web::block(move || {
        let mut connection = pool.get()?;
        return delete_one(&mut connection, user_id);
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

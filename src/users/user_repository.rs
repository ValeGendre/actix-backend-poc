
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;



use crate::users::models;
use crate::users::models::user::User;
use crate::users::models::new_user::NewUser;
use crate::users::models::update_user::UpdateUser;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create(connection: &mut PgConnection, create_user_dto: &NewUser) -> Result<User, DbError> {
    use actix_backend::schema::users;

    let new_user = diesel::insert_into(users::table)
        .values(create_user_dto)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    Ok(new_user)
}

pub fn find_all(connection: &mut PgConnection) -> Result<Vec<User>, DbError> {
    use actix_backend::schema::users::dsl::*;

    let user_list = diesel::QueryDsl::select(users, User::as_select())
        .load(connection)
        .expect("Error loading users");
    
    Ok(user_list)
}

pub fn find_one(connection: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    use actix_backend::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<models::user::User>(connection)
        .expect("Error loading users");

    Ok(user)
}

pub fn update_one(connection: &mut PgConnection, user_id:i32, update_user_dto: &UpdateUser) -> Result<(), DbError> {
    use actix_backend::schema::users::dsl::*;

    diesel::update(users.find(user_id))
        .set(update_user_dto).execute(connection)
        .expect("Error updating users");

    Ok(())
}

pub fn delete_one(connection: &mut PgConnection, user_id:i32) -> Result<(), DbError> {
    use actix_backend::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(connection)
        .expect("Error deleting user");

    Ok(())
}
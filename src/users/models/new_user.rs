use diesel::prelude::Insertable;


#[derive(Insertable)]
#[diesel(table_name = actix_backend::schema::users)]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    pub administrator: &'a bool,
}
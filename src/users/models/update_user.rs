use diesel::query_builder::AsChangeset;


#[derive(AsChangeset)]
#[diesel(table_name = actix_backend::schema::users)]
pub struct UpdateUser<'a> {
    pub firstname: Option<&'a str>,
    pub lastname: Option<&'a str>,
    pub email: Option<&'a str>,
    pub administrator: Option<&'a bool>,
}
use diesel::Queryable;
use diesel::Insertable;
use crate::schema::invites;

#[derive(Queryable, Identifiable, AsChangeset)]
#[table_name="invites"]
pub struct Invite {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}


#[derive(Insertable)]
#[table_name="invites"]
pub struct NewInvite<'a> {
    pub name: &'a str,
    pub phone: &'a str,
}

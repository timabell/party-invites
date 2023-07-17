#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;
mod db;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            routes::index::index,
            // routes::events::events,
            // routes::rsvp::rsvp,
            // routes::sms::send_sms
        ],
    )
}

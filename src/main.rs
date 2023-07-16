#[macro_use] extern crate rocket;
mod models;
mod routes;

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

#[macro_use] extern crate rocket;

// mod plugins;//::http::HttpDestination;
// use plugins::http::HttpDestination;

// mod event_store;
// use event_store::EventStore;

struct EventScheduleRequest {
    event_body: String,
    schedule_time: i64,
    destination: String,//HttpDestination,
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

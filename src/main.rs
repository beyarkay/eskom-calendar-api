#[macro_use]
extern crate rocket;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
mod versions;

use versions::*;
mod structs;

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
/// https://stackoverflow.com/a/72702246/14555505
#[options("/<_..>")]
fn all_options() {}

/// An empty struct to trigger CORS
/// https://stackoverflow.com/a/72702246/14555505
pub struct Cors;

/// Implement Fairing so that CORS issues don't come up
/// https://stackoverflow.com/a/72702246/14555505
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .attach(Cors)
        .mount(
            "/",
            routes![
                latest::fuzzy_search,
                latest::list_all_areas,
                latest::list_areas,
                latest::outages,
                latest::schedules,
            ],
        )
        .mount(
            "/v0.0.1",
            routes![
                v0_0_1::fuzzy_search,
                v0_0_1::list_all_areas,
                v0_0_1::list_areas,
                v0_0_1::outages,
                v0_0_1::schedules,
            ],
        );

    Ok(rocket.into())
}

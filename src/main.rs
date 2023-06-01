#[macro_use]
extern crate rocket;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    serde::json::Json,
    Request, Response,
};
mod versions;

use versions::*;
mod structs;
use structs::schedule::PowerOutage;

#[get("/area_search/<query>")]
async fn _area_search(query: String) -> Result<Json<Vec<PowerOutage>>, &'static str> {
    let _ = query;
    let url = "https://raw.githubusercontent.com/beyarkay/eskom-calendar/main/area_metadata.yaml";
    let _text_data = reqwest::get(url)
        .await
        .map_err(|_err| "Failed to get area_metadata.yaml")?
        .text()
        .await
        .map_err(|_err| "Failed to get text of area_metadata.yaml")?;

    Ok(Json(vec![]))
}

async fn get_machine_friendly() -> Result<Vec<PowerOutage>, String> {
    let url =
        "https://github.com/beyarkay/eskom-calendar/releases/download/latest/machine_friendly.csv";
    let text_data = reqwest::get(url)
        .await
        .map_err(|_err| "Failed to get machine_friendly.csv that defines the outages")?
        .text()
        .await
        .map_err(|_err| "Failed to get text of machine_friendly.csv")?;

    let mut reader = csv::Reader::from_reader(text_data.as_bytes());
    Ok(reader
        .deserialize::<PowerOutage>()
        .map(|result| result.unwrap())
        .collect())
}

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
                latest::outages,
                latest::schedules,
                latest::list_areas,
                latest::list_all_areas
            ],
        )
        .mount(
            "/v0.0.1",
            routes![
                v0_0_1::outages,
                v0_0_1::schedules,
                v0_0_1::list_areas,
                v0_0_1::list_all_areas
            ],
        );

    Ok(rocket.into())
}

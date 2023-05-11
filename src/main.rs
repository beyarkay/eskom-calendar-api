// use rocket::fs::FileServer;
use rocket::serde::json::Json;

mod structs;
use structs::{
    schedule::PowerOutage,
    status::{EspStatus, EspStatusOuter, MunicipalStatus, Stage},
};

#[macro_use]
extern crate rocket;

#[get("/schedule/<area_name>")]
async fn schedule(area_name: String) -> Result<Json<Vec<PowerOutage>>, &'static str> {
    let url =
        "https://github.com/beyarkay/eskom-calendar/releases/download/latest/machine_friendly.csv";
    let text_data = reqwest::get(url)
        .await
        .map_err(|_err| "Failed to get machine_friendly")?
        .text()
        .await
        .map_err(|_err| "Failed to get text of machine_friendly.csv")?;

    let mut reader = csv::Reader::from_reader(text_data.as_bytes());
    let outages: Vec<PowerOutage> = reader
        .deserialize::<PowerOutage>()
        .map(|result| result.unwrap())
        .filter(|outage| outage.area_name == area_name)
        .collect();

    Ok(Json(outages))
}

#[get("/")]
fn esp_index() -> &'static str {
    "Oh hello there, EskomSePush"
}

#[get("/status")]
fn esp_status() -> Json<EspStatusOuter> {
    Json(EspStatusOuter {
        status: EspStatus {
            capetown: Some(MunicipalStatus {
                name: "Cape Town".to_string(),
                next_stages: vec![
                    Stage {
                        stage: "1".to_string(),
                        stage_start_timestamp: "2022-08-08T17:00:00+02:00".to_string(),
                    },
                    Stage {
                        stage: "0".to_string(),
                        stage_start_timestamp: "2022-08-08T22:00:00+02:00".to_string(),
                    },
                ],
                stage: "0".to_string(),
                stage_updated: "2022-08-08T00:08:16.837063+02:00".to_string(),
            }),
            eskom: Some(MunicipalStatus {
                name: "National".to_string(),
                next_stages: vec![
                    Stage {
                        stage: "2".to_string(),
                        stage_start_timestamp: "2022-08-08T16:00:00+02:00".to_string(),
                    },
                    Stage {
                        stage: "0".to_string(),
                        stage_start_timestamp: "2022-08-09T00:00:00+02:00".to_string(),
                    },
                ],
                stage: "0".to_string(),
                stage_updated: "2022-08-08T16:12:53.725852+02:00".to_string(),
            }),
        },
    })
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![schedule]);
        // .mount("/csv/", FileServer::from("generated/"))
        // .mount("/esp/2.0", routes![esp_index, esp_status]);

    Ok(rocket.into())
}


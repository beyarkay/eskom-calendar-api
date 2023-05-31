#[macro_use]
extern crate rocket;
use std::collections::HashSet;

use rocket::serde::json::Json;
mod structs;
use regex::Regex;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use structs::{
    esp::{MunicipalStatus, Stage, StatusInner, StatusOuter},
    new::{
        RawMonthlyShedding, RawPeriodicShedding, RawWeeklyShedding, RecurringOutage,
        RecurringSchedule, ScheduleId,
    },
    schedule::PowerOutage,
};

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

#[get("/schedules/<area_name>")]
async fn schedules(area_name: String) -> Result<Json<RecurringSchedule>, String> {
    let url = format!(
        "https://raw.githubusercontent.com/beyarkay/eskom-calendar/main/generated/{area_name}.csv"
    );
    let response = reqwest::get(url)
        .await
        .map_err(|_err| format!("Failed to get CSV file defining schedules for {area_name}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to get CSV file from GitHub: {:?}",
            response
        ));
    }

    let text_data = response.text().await.map_err(|_err| {
        format!("Failed to get text of the CSV file defining schedules for {area_name}")
    })?;

    let mut reader = csv::Reader::from_reader(text_data.as_bytes());
    let headers = reader
        .headers()
        .map_err(|_err| "Couldn't read headers for CSV file")?;
    let outages: Vec<RecurringOutage>;

    // Parse the CSV file in a manner that depends on the headers
    if headers.iter().any(|h| h == "date_of_month") {
        outages = reader
            .deserialize::<RawMonthlyShedding>()
            .map(|res| Into::<RecurringOutage>::into(res.unwrap()))
            .collect::<Vec<_>>();
    } else if headers.iter().any(|h| h == "day_of_week") {
        outages = reader
            .deserialize::<RawWeeklyShedding>()
            .map(|res| Into::<RecurringOutage>::into(res.unwrap()))
            .collect::<Vec<_>>();
    } else if headers.iter().any(|h| h == "day_of_20_day_cycle") {
        outages = reader
            .deserialize::<RawPeriodicShedding>()
            .map(|res| Into::<RecurringOutage>::into(res.unwrap()))
            .collect::<Vec<_>>();
    } else {
        return Err(format!("Couldn't parse headers {:?}", headers));
    }

    // TODO actually assign values for id, source, info, last_updated, valid_from, valid_until
    Ok(Json(RecurringSchedule {
        id: ScheduleId(0),
        outages,
        source: vec![],
        info: vec![],
        last_updated: None,
        valid_from: None,
        valid_until: None,
    }))
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

#[get("/outages/<area_name>")]
async fn outages(area_name: String) -> Result<Json<Vec<PowerOutage>>, String> {
    let outages: Vec<PowerOutage> = get_machine_friendly()
        .await?
        .into_iter()
        .filter(|outage| outage.area_name == area_name)
        .collect();

    if outages.is_empty() {
        return Err(format!("No areas found that match `{area_name}`"));
    }

    Ok(Json(outages))
}

#[get("/list_areas")]
async fn list_all_areas() -> Result<Json<Vec<String>>, String> {
    list_areas(".*".to_string()).await
}

#[get("/list_areas/<regex>")]
async fn list_areas(regex: String) -> Result<Json<Vec<String>>, String> {
    let machine_friendly = get_machine_friendly().await?;
    let re = Regex::new(&regex).map_err(|e| format!("Error parsing '{regex}' as regex: {e:?}"))?;

    let mut uniq_areas = machine_friendly
        .into_iter()
        .filter(|outage| re.is_match(&outage.area_name))
        .map(|outage| outage.area_name)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    uniq_areas.sort();

    Ok(Json(uniq_areas))
}

#[get("/")]
fn esp_index() -> &'static str {
    "Oh hello there, EskomSePush"
}

#[get("/status")]
fn esp_status() -> Json<StatusOuter> {
    Json(StatusOuter {
        status: StatusInner {
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

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_aws_rds::Postgres()] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    eprintln!("Including schema");
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = MyState { pool };
    eprintln!("Building rocket");
    let rocket = rocket::build()
        .mount(
            "/v0.0.1",
            routes![outages, schedules, list_areas, list_all_areas],
        )
        // .mount("/esp/2.0", routes![esp_index, esp_status])
        .manage(state);

    Ok(rocket.into())
}

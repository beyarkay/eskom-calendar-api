use crate::structs::new::{
    RawMonthlyShedding, RawPeriodicShedding, RawWeeklyShedding, RecurringOutage, RecurringSchedule,
    ScheduleId,
};
use crate::{get_machine_friendly, structs::schedule::PowerOutage};
use rocket::serde::json::Json;
use std::collections::HashSet;

use regex::Regex;

pub mod latest {
    use super::*;
    #[get("/outages/<area_name>")]
    pub async fn outages(area_name: String) -> Result<Json<Vec<PowerOutage>>, String> {
        super::v0_0_1::outages(area_name).await
    }

    #[get("/schedules/<area_name>")]
    pub async fn schedules(area_name: String) -> Result<Json<RecurringSchedule>, String> {
        super::v0_0_1::schedules(area_name).await
    }

    #[get("/list_areas")]
    pub async fn list_all_areas() -> Result<Json<Vec<String>>, String> {
        super::v0_0_1::list_all_areas().await
    }
    #[get("/list_areas/<regex>")]
    pub async fn list_areas(regex: String) -> Result<Json<Vec<String>>, String> {
        super::v0_0_1::list_areas(regex).await
    }
}

pub mod v0_0_1 {

    use super::*;

    #[get("/outages/<area_name>")]
    pub async fn outages(area_name: String) -> Result<Json<Vec<PowerOutage>>, String> {
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

    #[get("/schedules/<area_name>")]
    pub async fn schedules(area_name: String) -> Result<Json<RecurringSchedule>, String> {
        let url = format!( "https://raw.githubusercontent.com/beyarkay/eskom-calendar/main/generated/{area_name}.csv");
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

    #[get("/list_areas")]
    pub async fn list_all_areas() -> Result<Json<Vec<String>>, String> {
        list_areas(".*".to_string()).await
    }

    #[get("/list_areas/<regex>")]
    pub async fn list_areas(regex: String) -> Result<Json<Vec<String>>, String> {
        let machine_friendly = get_machine_friendly().await?;
        let re =
            Regex::new(&regex).map_err(|e| format!("Error parsing '{regex}' as regex: {e:?}"))?;

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
}

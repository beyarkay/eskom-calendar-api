use crate::structs::{
    Area, AreaId, PowerOutage, RawMonthlyShedding, RawPeriodicShedding, RawWeeklyShedding,
    RecurringOutage, RecurringSchedule, ScheduleId, SearchResult,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use regex::Regex;
use rocket::serde::json::Json;
use shuttle_runtime::tracing;
use std::collections::HashSet;

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

pub mod latest {
    use super::*;

    #[get("/fuzzy_search/<query>")]
    pub async fn fuzzy_search(query: String) -> Result<Json<Vec<SearchResult<Area>>>, String> {
        super::v0_0_1::fuzzy_search(query).await
    }

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

    #[get("/fuzzy_search/<query>")]
    pub async fn fuzzy_search(query: String) -> Result<Json<Vec<SearchResult<Area>>>, String> {
        tracing::info!("Fuzzy searching on {query}");
        let matcher = SkimMatcherV2::default();

        // Normalise a query
        let preprocess = |q: &str| {
            // Replace all non a-z0-9_ chars with a space to aid in fuzzy matching
            let re = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
            re.replace_all(q, " ").to_ascii_lowercase()
        };

        // Normalise the query
        let query = preprocess(&query);
        tracing::info!("Fetching machine friendly");
        let machine_friendly = get_machine_friendly().await?;

        tracing::info!("Fuzzy searching for matching areas");
        // Find all matching areas
        let mut matching_areas = machine_friendly
            .into_iter()
            .map(|outage| outage.area_name)
            .collect::<HashSet<_>>()
            .into_iter()
            .filter_map(|area_name| {
                matcher
                    .fuzzy_match(&preprocess(&area_name), &preprocess(&query))
                    .map(|score| SearchResult {
                        score,
                        result: Area {
                            name: area_name,
                            id: AreaId(0),
                            schedule: ScheduleId(0),
                            aliases: vec![],
                            province: None,
                            municipality: None,
                        },
                    })
            })
            .collect::<Vec<_>>();

        tracing::info!("Sorting matching areas");
        matching_areas.sort();
        matching_areas.reverse();

        tracing::info!("Returning result");
        Ok(Json(matching_areas))
    }

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

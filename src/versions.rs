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
use tracing::Instrument;

async fn get_machine_friendly() -> Result<Vec<PowerOutage>, String> {
    let machine_friendly_span = tracing::info_span!("Getting machine friendly");
    let _ = machine_friendly_span.enter();

    let fetch_span = tracing::info_span!("Making GET request to GitHub");
    let convert_span = tracing::info_span!("Converting GitHub reponse to text");
    let url =
        "https://github.com/beyarkay/eskom-calendar/releases/download/latest/machine_friendly.csv";
    let text_data = reqwest::get(url)
        .instrument(fetch_span)
        .await
        .map_err(|_err| "Failed to get machine_friendly.csv that defines the outages")?
        .text()
        .instrument(convert_span)
        .await
        .map_err(|_err| "Failed to get text of machine_friendly.csv")?;

    tracing::info!("Parsing machine_friendly.csv");
    let mut reader = csv::Reader::from_reader(text_data.as_bytes());
    Ok(reader
        .deserialize::<PowerOutage>()
        .map(|result| result.unwrap())
        .collect())
}

pub mod latest {
    use super::*;

    pub fn routes() -> Vec<rocket::Route> {
        routes![fuzzy_search, list_all_areas, list_areas, outages, schedules,]
    }

    /// Search for an area using approximate (or "fuzzy") matching.
    ///
    /// For example, `west dorp` will match all areas that have `west` and `dorp` in their names in
    /// that order. This is useful if you don't know what eskom-calendar calls the area you are in.
    ///
    /// The returned `score` describes how good a match each item is. The higher the score, the
    /// better the match. Click 'Try it out' on the right to have a go!
    #[utoipa::path(
        params(("query" = String, example="west dorp", description = "Space separated search queryies (order matters)")),
        responses(
            (status = 200, description = "Success. You'll get a list of search results", body = [SearchResult])
        ),
    )]
    #[get("/fuzzy_search/<query>")]
    pub async fn fuzzy_search(query: String) -> Result<Json<Vec<SearchResult<Area>>>, String> {
        super::v0_0_1::fuzzy_search(query).await
    }

    /// Get all the known times when power will be off for a certain area.
    ///
    /// The `area_name` must be one of the ones listed in the endpoint `list_areas`. Click 'Try it
    /// out' on the right to have a go!
    #[utoipa::path(
        params(("area_name" = String, example="western-cape-stellenbosch", description = "Area to get the outages for")),
        responses(
            (status = 200, description = "200 will return a list of PowerOutage objects.", body = [PowerOutage])
        ),
    )]
    #[get("/outages/<area_name>")]
    pub async fn outages(area_name: String) -> Result<Json<Vec<PowerOutage>>, String> {
        super::v0_0_1::outages(area_name).await
    }

    /// Get the loadshedding schedule for a certain area.
    ///
    /// Note that this does *not* describe when the power will be off (use `/outages/{area_name}`
    /// instead). The `area_name` must be one of the ones listed in the endpoint `list_areas`.
    /// Click 'Try it out' on the right to have a go!
    #[utoipa::path(
        params(("area_name" = String, example="north-west-zeerust", description = "The name of the area you want the schedule for")),
        responses(
            (status = 200, description = "Success. You'll get a Recurring Schedule", body = RecurringSchedule)
        ),
    )]
    #[get("/schedules/<area_name>")]
    pub async fn schedules(area_name: String) -> Result<Json<RecurringSchedule>, String> {
        super::v0_0_1::schedules(area_name).await
    }

    /// Get a list of all areas known to eskom-calendar.
    ///
    /// Each area name is unique, and describes a different `Area` that can get loadshedding. Click
    /// 'Try it out' on the right to have a go!
    #[utoipa::path(responses(
        (status = 200, description = "Success. A list of every area known to eskom-calendar.", body = [String])
    ))]
    #[get("/list_areas")]
    pub async fn list_all_areas() -> Result<Json<Vec<String>>, String> {
        super::v0_0_1::list_all_areas().await
    }

    /// Search for areas by a rust-regex.
    ///
    /// Have a look [here](https://regex101.com/r/XspP8R/1) to try out your query on a long list of
    /// areas. Don't forget to [URI escape](https://en.wikipedia.org/wiki/URL_encoding) your query
    /// before you try to send it. Click 'Try it out' on the right to have a go!
    #[utoipa::path(
        params(("regex" = String, example="\\w+(ville|water)", description = "Valid Rust regex describing the place you're looking for.")),
        responses(
            (status = 200, description = "Success. You'll get a list of areas matching your regex", body = [String])
        ),
    )]
    #[get("/list_areas/<regex>")]
    pub async fn list_areas(regex: String) -> Result<Json<Vec<String>>, String> {
        super::v0_0_1::list_areas(regex).await
    }
}

pub mod v0_0_1 {

    use super::*;

    pub fn routes() -> Vec<rocket::Route> {
        routes![fuzzy_search, list_all_areas, list_areas, outages, schedules,]
    }

    #[utoipa::path(context_path = "/v0.0.1")]
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

        // Get the machine friendly data
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

    #[utoipa::path(context_path = "/v0.0.1")]
    #[get("/outages/<area_name>")]
    pub async fn outages(area_name: String) -> Result<Json<Vec<PowerOutage>>, String> {
        tracing::info!("Getting outages for {area_name}");
        let outages: Vec<PowerOutage> = get_machine_friendly()
            .await?
            .into_iter()
            .filter(|outage| outage.area_name == area_name)
            .collect();

        if outages.is_empty() {
            tracing::info!("No outages found for {area_name}");
            return Err(format!("No areas found that match `{area_name}`"));
        }

        tracing::info!("Returning outages for {area_name}");
        Ok(Json(outages))
    }

    #[utoipa::path(context_path = "/v0.0.1")]
    #[get("/schedules/<area_name>")]
    pub async fn schedules(area_name: String) -> Result<Json<RecurringSchedule>, String> {
        tracing::info!("Getting schedules for {area_name}");
        let url = format!( "https://raw.githubusercontent.com/beyarkay/eskom-calendar/main/generated/{area_name}.csv");
        let response = reqwest::get(url)
            .await
            .map_err(|_err| format!("Failed to get CSV file defining schedules for {area_name}"))?;

        tracing::info!("Checking if GitHub request was successful");
        if !response.status().is_success() {
            return Err(format!(
                "Failed to get CSV file from GitHub: {:?}",
                response
            ));
        }

        let text_data = response.text().await.map_err(|_err| {
            format!("Failed to get text of the CSV file defining schedules for {area_name}")
        })?;

        tracing::info!("Parsing schedule CSV as text");
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

        tracing::info!("Returning parsed CSV as a RecurringSchedule");
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

    #[utoipa::path(context_path = "/v0.0.1")]
    #[get("/list_areas")]
    pub async fn list_all_areas() -> Result<Json<Vec<String>>, String> {
        list_areas(".*".to_string()).await
    }

    #[utoipa::path(context_path = "/v0.0.1")]
    #[get("/list_areas/<regex>")]
    pub async fn list_areas(regex: String) -> Result<Json<Vec<String>>, String> {
        tracing::info!("Listing all areas matching the regex `{regex}`");
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

        tracing::info!("Sorting the areas");
        uniq_areas.sort();

        tracing::info!("Returning the sorted areas");
        Ok(Json(uniq_areas))
    }
}

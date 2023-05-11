pub mod status {
    use rocket::serde::Serialize;
    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct EspStatusOuter {
        pub status: EspStatus,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct EspStatus {
        pub capetown: Option<MunicipalStatus>,
        pub eskom: Option<MunicipalStatus>,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct MunicipalStatus {
        pub name: String,
        pub next_stages: Vec<Stage>,
        pub stage: String,
        pub stage_updated: String,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Stage {
        pub stage: String,
        pub stage_start_timestamp: String,
    }
}

pub mod area_information {
    use rocket::serde::Serialize;

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct AreaInformation {
        events: Vec<Event>,
        info: Info,
        schedule: Schedule,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Event {
        end: String,
        note: String,
        start: String,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Info {
        name: String,
        region: String,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Schedule {
        days: Vec<DayOfLoadshedding>,
        source: String,
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct DayOfLoadshedding {
        date: String,
        name: String,
        stages: Vec<Vec<String>>,
    }
}

pub mod schedule {

    use chrono::{DateTime, FixedOffset};
    use rocket::serde::{Deserialize,Serialize};

    /// Represents a duration of time for which the power will be out for a particular area.
    ///
    /// Requires specifying where the information came from (in `source`) as well as the stage of
    /// loadshedding.
    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    #[derive(PartialEq, Eq, Clone)]
    pub struct PowerOutage {
        pub area_name: String,
        pub stage: u8,
        pub start: DateTime<FixedOffset>,
        pub finsh: DateTime<FixedOffset>,
        pub source: String,
    }
}

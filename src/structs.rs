use std::{cmp::Ordering, fmt::Debug};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub enum Errors {
    /// Unfortunately there's gotta be a default catch-all error
    Unspecified(String),
}

/// The unique ID of a schedule
#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ScheduleId(pub i64);

/// A loadshedding schedule that repeats over some period.
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct RecurringSchedule {
    pub id: ScheduleId,

    /// All the recurring outages for this schedule.
    #[schema(example = "[RecurringOutage]")]
    pub outages: Vec<RecurringOutage>,

    /// The source where this schedule came from. Often this information is distributed by the
    /// municipality of the area, or Eskom itself. This might come from multiple places
    #[schema(
        example = "[\"https://www.eskom.co.za/distribution/wp-content/uploads/2022/09/WesternCape_LS.xlsx\"]"
    )]
    pub source: Vec<String>,

    /// Often the `source` will be a PDF or Excel spreadsheet, without any context or other
    /// information. So `info` has any relevant URLs that describe the `source`.
    #[schema(
        example = "[\"https://www.eskom.co.za/distribution/customer-service/outages/downloadable-loadshedding-spreadsheets-for-eskom-customers/\"]"
    )]
    pub info: Vec<String>,

    /// The last time this schedule was updated. In case a new schedule has recently been released,
    /// this will let you know if eskom-calendar has been updated with these schedules.
    #[schema(example = "2000-01-01T00:00:00+02:00")]
    pub last_updated: Option<NaiveDateTime>,

    /// The start date which this schedule is valid from. Sometimes a new schedule will be
    /// announced in advanced but will only come into action on a certain date.
    #[schema(example = "2000-01-01T00:00:00+02:00")]
    pub valid_from: Option<NaiveDateTime>,

    /// The final date which this schedule is valid until. Sometimes a new schedule will be
    /// deprecated in favour of some other new schedule.
    #[schema(example = "2099-01-01T00:00:00+02:00")]
    pub valid_until: Option<NaiveDateTime>,
}

/// A recurring time during which the power *could* be out.
///
/// Note that this is *different* to `PowerOutage`. A recurring outage does not describe a time
/// when your power will be out, but rather describes a time when your power *could* be out,
/// depending on what stage of loadshedding is announced.
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct RecurringOutage {
    /// The time at which this outage starts
    #[schema(example = "22:00")]
    pub start_time: NaiveTime,

    /// The time at which this outage finishs.
    ///
    /// Note that this is spelt `finsh`, without the second `i`, so that it lines up with `start`.
    /// It *is* possible for `finsh_time` < `start_time` (for example `00:30` < `22:00`), and this
    /// happens when loadshedding goes over midnight.
    #[schema(example = "00:30")]
    pub finsh_time: NaiveTime,

    /// The loadshedding stage.
    #[schema(example = 3)]
    pub stage: u8,

    /// How often this outage is repeated. Most common are Monthly and Weekly.
    #[schema(example = "Monthly")]
    pub recurrence: Recurrence,

    /// The day of this recurrence, starting from 1.
    ///
    /// Since a `RecurringOutage` could repeat every month/week/other time period, this describes
    /// what day of the month/week/other time period this outage starts on.
    ///
    /// - 1 => 1st of the month, Monday
    /// - 2 => 2nd of the month, Tuesday
    /// - 3 => 3rd of the month, Wednesday
    /// - etc
    #[schema(example = 4)]
    pub day1_of_recurrence: u8,
}

/// An enum to describe either a Weekly, Monthly, or (most general) Periodic recurrance.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
#[serde(crate = "rocket::serde")]
pub enum Recurrence {
    /// Repeat every week
    Weekly,
    /// Repeat every month
    Monthly,
    /// Repeat with a period of `period_days` days, starting from the date `offset`
    Periodic { offset: NaiveDate, period_days: u8 },
}

/// The ID of an `Area`
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct AreaId(pub i64);

/// A geographical area which has a loadshedding schedule. Note that multiple Areas might share the
/// same schedule.
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct Area {
    /// The standardised name of this area
    pub name: String,
    /// The ID of this area
    pub id: AreaId,
    /// The ID of the schedule that this area follows
    pub schedule: ScheduleId,
    /// Different aliases for this area. These might be common misspellings, the area's name in
    /// different languages, or local nicknames for the area
    pub aliases: Vec<String>,
    /// The province of this area (not always known, so it might be None)
    pub province: Option<Province>,
    /// The municipality of this area (not always known, so it might be None)
    pub municipality: Option<Municipality>,
}

/// A region on the surface of Earth that is fully connected. So you can't have two "islands",
/// every point in a ContiguousRegion must be reachable from every other point in the same
/// ContiguousRegion.
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ContiguousRegion {
    boundary: Vec<Coords>,
}

/// A point on the earth
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct Coords {
    lat: f64,
    lng: f64,
}

/// One of the nine provinces of South Africa
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub enum Province {
    EasternCape,
    FreeState,
    Gauteng,
    KwaZuluNatal,
    Limpopo,
    Mpumalanga,
    NorthWest,
    NorthernCape,
    WesternCape,
}

/// Municipalities in South Africa can either be Metropolitan Municipalities, or they can be
/// District Municipalities (in which case they are subdivided into Local Municipalities).
///
/// The metro municipalities are generally high density cities and the surrounding areas, and the
/// district municipalities are everywhere else.
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub enum Municipality {
    Metro(MetroMunic),
    District {
        district: DistrictMunic,
        local: LocalMunic,
    },
}

/// All the Metropolitan Municipalities in South Africa
///
/// https://en.wikipedia.org/wiki/List_of_municipalities_in_South_Africa#Metropolitan_municipalities
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
#[allow(non_camel_case_types)]
pub enum MetroMunic {
    BuffaloCity,
    CityOfCapeTown,
    CityOfEkurhuleni,
    CityOfJohannesburg,
    CityOfTshwane,
    Mangaung,
    NelsonMandelaBay,
    eThekwini,
}

/// All the district municipalities in South Africa
///
/// https://en.wikipedia.org/wiki/List_of_municipalities_in_South_Africa#Local_municipalities
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
#[allow(non_camel_case_types)]
pub enum DistrictMunic {
    AlfredNzo,
    Amajuba,
    Amathole,
    Bojanala,
    CapeWinelands,
    Capricorn,
    CentralKaroo,
    ChrisHani,
    DrKennethKaunda,
    DrRuthSegomotsiMompati,
    Ehlanzeni,
    FezileDabi,
    FrancesBaard,
    GardenRoute,
    GertSibande,
    HarryGwala,
    JoeGqabi,
    JohnTaoloGaetsewe,
    KingCetshwayo,
    Lejweleputswa,
    Mopani,
    Namakwa,
    NgakaModiriMolema,
    Nkangala,
    ORTambo,
    Overberg,
    PixleykaSeme,
    SarahBaartman,
    Sedibeng,
    Sekhukhune,
    ThaboMofutsanyana,
    Ugu,
    Vhembe,
    Waterberg,
    WestCoast,
    WestRand,
    Xhariep,
    ZFMgcawu,
    Zululand,
    iLembe,
    uMgungundlovu,
    uMkhanyakude,
    uMzinyathi,
    uThukela,
}

/// All Local Municipalities of South Africa.
///
/// https://en.wikipedia.org/wiki/List_of_municipalities_in_South_Africa#Local_municipalities
#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
#[allow(non_camel_case_types)]
pub enum LocalMunic {
    Abaqulusi,
    AlbertLuthuli,
    AlfredDuma,
    Amahlathi,
    BaPhalaborwa,
    BeaufortWest,
    BelaBela,
    Bergrivier,
    BigFiveHlabisa,
    Bitou,
    Blouberg,
    BlueCraneRoute,
    BreedeValley,
    Bushbuckridge,
    CapeAgulhas,
    Cederberg,
    CityOfMatlosana,
    CollinsChabane,
    Dannhauser,
    DawidKruiper,
    Dihlabeng,
    Dikgatlong,
    Dipaleseng,
    Ditsobotla,
    DrBeyersNaude,
    DrJSMoroka,
    DrNkosazanaDlaminiZuma,
    Drakenstein,
    EliasMotsoaledi,
    Elundini,
    Emakhazeni,
    EmalahleniEasternCape,
    EmalahleniMpumalanga,
    Emfuleni,
    Emthanjeni,
    Endumeni,
    Engcobo,
    EnochMgijima,
    EphraimMogale,
    FetakgomoTubatse,
    GaSegonyana,
    Gamagara,
    George,
    GovanMbeki,
    GreatKei,
    GreaterGiyani,
    GreaterKokstad,
    GreaterLetaba,
    GreaterTaung,
    GreaterTzaneen,
    Hantam,
    Hessequa,
    Impendle,
    IngquzaHill,
    InkosiLangalibalele,
    IntsikaYethu,
    InxubaYethemba,
    JBMarks,
    JoeMorolong,
    Jozini,
    KagisanoMolopo,
    KaiGarib,
    Kamiesberg,
    Kannaland,
    Kareeberg,
    KarooHoogland,
    Kgatelopele,
    Kgetlengrivier,
    KhaiMa,
    Kheis,
    KingSabataDalindyebo,
    Knysna,
    Kopanong,
    KouKamma,
    Kouga,
    KwaDukuza,
    Laingsburg,
    Langeberg,
    Lekwa,
    LekwaTeemane,
    LepelleNkumpi,
    Lephalale,
    Lesedi,
    Letsemeng,
    Madibeng,
    Mafube,
    Magareng,
    Mahikeng,
    Makana,
    Makhado,
    Makhuduthamaga,
    MalutiAPhofung,
    Mamusa,
    Mandeni,
    Mantsopa,
    Maphumulo,
    MaquassiHills,
    Maruleng,
    Masilonyana,
    Matatiele,
    Matjhabeng,
    Matzikama,
    Mbhashe,
    Mbombela,
    MerafongCity,
    Metsimaholo,
    Mhlontlo,
    Midvaal,
    Mkhambathini,
    Mkhondo,
    Mnquma,
    ModimolleMookgophong,
    Mogalakwena,
    MogaleCity,
    Mohokare,
    Molemole,
    Moqhaka,
    Moretele,
    MosesKotane,
    MosselBay,
    Mpofana,
    Msinga,
    Msukaligwa,
    Msunduzi,
    Mthonjaneni,
    Mtubatuba,
    Musina,
    Nala,
    Naledi,
    NamaKhoi,
    Ndlambe,
    Ndwedwe,
    Newcastle,
    Ngqushwa,
    Ngwathe,
    Nkandla,
    Nketoana,
    Nkomazi,
    Nongoma,
    Nqutu,
    Ntabankulu,
    Nyandeni,
    Okhahlamba,
    Oudtshoorn,
    Overstrand,
    Phokwane,
    Phumelela,
    PixleykaSeme,
    Polokwane,
    PortStJohns,
    PrinceAlbert,
    RamotshereMoiloa,
    RandWestCity,
    Ratlou,
    RayNkonyeni,
    RaymondMhlaba,
    Renosterberg,
    Richmond,
    Richtersveld,
    Rustenburg,
    Sakhisizwe,
    SaldanhaBay,
    Senqu,
    Setsoto,
    Siyancuma,
    Siyathemba,
    SolPlaatje,
    Stellenbosch,
    SteveTshwete,
    SundaysRiverValley,
    Swartland,
    Swellendam,
    ThabaChweu,
    Thabazimbi,
    Theewaterskloof,
    Thembelihle,
    ThembisileHani,
    Thulamela,
    Tokologo,
    Tsantsabane,
    Tswaing,
    Tswelopele,
    Ubuhlebezwe,
    Ubuntu,
    Ulundi,
    Umdoni,
    Umsobomvu,
    Umvoti,
    Umzimkhulu,
    Umzimvubu,
    Umzumbe,
    VictorKhanye,
    WalterSisulu,
    WinnieMadikizelaMandela,
    Witzenberg,
    eDumbe,
    eMadlangeni,
    uMfolozi,
    uMhlabuyalingana,
    uMhlathuze,
    uMlalazi,
    uMngeni,
    uMshwathi,
    uMuziwabantu,
    uPhongolo,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RawPeriodicShedding {
    /// The time when LoadShedding *should* start.
    pub start_time: String,
    /// The time when LoadShedding *should* finish (note the spelling).
    pub finsh_time: String,
    /// The stage of loadshedding.
    pub stage: u8,
    /// The day of the 20 day cycle, with the first day being 1, the second day being 2, etc
    pub day_of_cycle: u8,
    pub period_of_cycle: u8,
    pub start_of_cycle: String,
}

impl From<RawPeriodicShedding> for RecurringOutage {
    fn from(raw: RawPeriodicShedding) -> Self {
        assert!(
            raw.day_of_cycle <= raw.period_of_cycle,
            "Day of the cycle {} must be <= period of the cycle {}",
            raw.day_of_cycle,
            raw.period_of_cycle
        );

        let offset = NaiveDate::parse_from_str(&raw.start_of_cycle, "%Y-%m-%d").unwrap();

        RecurringOutage {
            start_time: NaiveTime::parse_from_str(&raw.start_time, "%H:%M").unwrap(),
            finsh_time: NaiveTime::parse_from_str(&raw.finsh_time, "%H:%M").unwrap(),
            stage: raw.stage,
            recurrence: Recurrence::Periodic {
                // As declared by
                // https://nelsonmandelabay.gov.za/DataRepository/Documents/residentialfull18febtojun11-2023_oYkoI.pdf
                // and also https://nelsonmandelabay.gov.za/documentslist?searchtext=&categoryid=58
                // LoadShedding information is available here:
                // https://nelsonmandelabay.gov.za/page/loadshedding
                offset,
                period_days: raw.period_of_cycle,
            },
            day1_of_recurrence: raw.day_of_cycle,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RawWeeklyShedding {
    /// The time when LoadShedding *should* start.
    pub start_time: String,
    /// The time when LoadShedding *should* finish (note the spelling).
    pub finsh_time: String,
    /// The stage of loadshedding.
    pub stage: u8,
    /// The day of the week, with Monday being 1, Tuesday being 2, etc
    pub day_of_week: u8,
}

impl From<RawWeeklyShedding> for RecurringOutage {
    fn from(raw: RawWeeklyShedding) -> Self {
        assert!(
            0 < raw.day_of_week && raw.day_of_week < 8,
            "Day of the week must be one of 1, 2, 3, 4, 5, 6, 7"
        );
        RecurringOutage {
            start_time: NaiveTime::parse_from_str(&raw.start_time, "%H:%M").unwrap(),
            finsh_time: NaiveTime::parse_from_str(&raw.finsh_time, "%H:%M").unwrap(),
            stage: raw.stage,
            recurrence: Recurrence::Weekly,
            day1_of_recurrence: raw.day_of_week,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RawMonthlyShedding {
    /// The time when LoadShedding *should* start.
    pub start_time: String,
    /// The time when LoadShedding *should* finish (note the spelling).
    pub finsh_time: String,
    /// The stage of loadshedding.
    pub stage: u8,
    /// The date of the month which this event occurs on
    pub date_of_month: u8,
}

impl From<RawMonthlyShedding> for RecurringOutage {
    fn from(raw: RawMonthlyShedding) -> Self {
        assert!(
            0 < raw.date_of_month && raw.date_of_month <= 31,
            "Date of month must be in the range (0, 31]"
        );

        RecurringOutage {
            start_time: NaiveTime::parse_from_str(&raw.start_time, "%H:%M").unwrap(),
            finsh_time: NaiveTime::parse_from_str(&raw.finsh_time, "%H:%M").unwrap(),
            stage: raw.stage,
            recurrence: Recurrence::Monthly,
            day1_of_recurrence: raw.date_of_month,
        }
    }
}

/// A duration in time when the power will be out for a certain area.
///
/// Note that this is different to `RecurringOutage`. A `PowerOutage` describes when your power
/// will actually be turned off, but a `RecurringOutage` describes the Monthly/Weekly schedules
/// that say "In this area, at this stage, on this date, your power will be off from this time to
/// that time".
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct PowerOutage {
    /// The area experiencing the power outage.
    #[schema(example = "western-cape-stellenbosch")]
    pub area_name: String,

    /// The stage of loadshedding
    #[schema(example = 5)]
    pub stage: u8,

    /// The datetime when loadshedding will start
    #[schema(example = "2023-06-01T20:00:00+02:00")]
    pub start: DateTime<FixedOffset>,

    /// The datetime when loadshedding will end. Note the spelling is `finsh`, not `finish` so that
    /// it lines up with `start`.
    #[schema(example = "2023-06-01T22:00:00+02:00")]
    pub finsh: DateTime<FixedOffset>,

    /// The source of information for this power outage. Useful for pointing fingers ;).
    #[schema(example = "https://twitter.com/Eskom_SA/status/1664250326818365440")]
    pub source: String,
}

/// A generic search result that gets returned after you searched for something.
///
/// It simply wraps the object you were looking for with a score for how well that object matched
/// your search query. Higher numbers are better.
#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct SearchResult<T> {
    /// How close a match the thing is to the search query. Higher is better.
    #[schema(example = 100)]
    pub score: i64,
    /// The thing that's been found
    #[schema(value_type=Area)]
    pub result: T,
}

impl<T> Ord for SearchResult<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<T> PartialOrd for SearchResult<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl<T> PartialEq for SearchResult<T> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T> Eq for SearchResult<T> {}

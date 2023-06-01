use std::{cmp::Ordering, fmt::Debug};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ScheduleId(pub i64);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RecurringSchedule {
    pub id: ScheduleId,
    pub outages: Vec<RecurringOutage>,
    pub source: Vec<String>,
    pub info: Vec<String>,
    pub last_updated: Option<NaiveDateTime>,
    pub valid_from: Option<NaiveDateTime>,
    pub valid_until: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RecurringOutage {
    pub start_time: NaiveTime,
    pub finsh_time: NaiveTime,
    pub stage: u8,
    pub recurrence: Recurrence,
    pub day1_of_recurrence: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug, Clone, PartialEq)]
pub enum Recurrence {
    Weekly,
    Monthly,
    Periodic { offset: NaiveDate, period_days: u8 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AreaId(pub i64);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Area {
    pub name: String,
    pub id: AreaId,
    pub schedule: ScheduleId,
    pub aliases: Vec<String>,
    pub province: Option<Province>,
    pub municipality: Option<Municipality>,
}

/// A region on the surface of Earth that is fully connected. So you can't have two "islands",
/// every point in a ContiguousRegion must be reachable from every other point in the same
/// ContiguousRegion.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ContiguousRegion {
    boundary: Vec<Coords>,
}

/// A point on the earth
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Coords {
    lat: f64,
    lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum Municipality {
    Metro(MetroMunic),
    District {
        district: DistrictMunic,
        local: LocalMunic,
    },
}

/// All the Metropolitan Municipalities in South Africa
/// https://en.wikipedia.org/wiki/List_of_municipalities_in_South_Africa#Metropolitan_municipalities
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(non_camel_case_types)]
pub enum MetroMunic {
    BuffaloCity,
    CityOfCapeTown,
    CityOfEkurhuleni,
    CityOfJohannesburg,
    CityOfTshwane,
    eThekwini,
    Mangaung,
    NelsonMandelaBay,
}

/// All the district municipalities in South Africa
/// https://en.wikipedia.org/wiki/List_of_municipalities_in_South_Africa#Local_municipalities
#[derive(Serialize, Deserialize, Debug)]
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
/// https://en.wikipedia.org/wiki/List_of_municipalities_in_South_Africa#Local_municipalities
#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

/// A loadshedding event that repeats on the same day every month, not yet parsed. See
/// MonthlyShedding.
#[derive(Deserialize, Debug)]
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

// TODO require that result is Ord
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone, Debug)]
pub struct SearchResult<T> {
    /// How close a match the thing is to the search query. Higher is better.
    pub score: i64,
    /// The thing that's been found
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

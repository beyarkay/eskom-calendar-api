#[macro_use]
extern crate rocket;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
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
    #[derive(OpenApi)]
    #[openapi(
        servers(
            (url = "https://eskom-calendar-api.shuttleapp.rs/", description = "Production Server (use this one)"),
            (url = "http://localhost:8000", description = "Development server"),
        ),
        info(
            title = "eskom-calendar API",
            contact(name = "Boyd Kane", url="https://twitter.com/beyarkay"),
            description = "\
            An easy-to-use, free, unrestricted API that gives South African developers access to \
            up-to-date loadshedding schedules and information. It's open source as well!\n\
            \n\
            See the source code for the API [here](https://github.com/beyarkay/eskom-calendar-api), the \
            source code for eskom-calendar itself \
            [here](https://github.com/beyarkay/eskom-calendar-api), and the official website at \
            [eskomcalendar.co.za](https://eskomcalendar.co.za). Leave a star! It helps us out (:\n\
            \n\
            ## Getting started\n\
            \n\
            This website is interactive and can make real requests, so you can test out the API in the \
            browser, no code required. Scroll down to one of the endpoints below, and expand the \
            options, and click the \"Try it out\" button on the right. There's plenty of \
            documentation so you should have no trouble getting up and running making API calls.\n\
            \n\
            But if you *really* just wanna check it out, here's how you can get the outages for \
            Stellenbosch, WC:\n\
            \n\
            ```sh\n\
            curl https://eskom-calendar-api.shuttleapp.rs/outages/western-cape-stellenbosch | jq\n\
            ```\n\
            And the response is just a JSON list:\n\
            ```json\n\
            [\n\
                {\n\
                    \"area_name\": \"western-cape-stellenbosch\",\n\
                    \"stage\": 6,\n\
                    \"start\": \"2023-06-01T18:00:00+02:00\",\n\
                    \"finsh\": \"2023-06-01T20:30:00+02:00\",\n\
                    \"source\": \"https://twitter.com/Eskom_SA/status/1664250326818365440\"\n\
                },\n\
                ...\n\
            ```\n\
            (Note that finish is spelt without the second `i`, so that it lines up with `start`)\n\
            \n\
            If you want to integrate this with your language of choice, OpenAPI auto-generated \
            libraries are on their way. Keep an eye out and follow Boyd on \
            [Twitter](https://twitter.com/beyarkay) for updates.\n\
            \n\
            ## Terminology\n\
            \n\
            There are basically two ideas when it comes to loadshedding: power outages and schedules.\n\
            \n\
            A *power outage* is a start and end time when power in a particular area will be off \
            due to loadshedding. This is the thing that you probably care about.\n\
            \n\
            A *schedule* is the big spreadsheet or PDF that Eskom/Cape Town/your municipality releases \
            that tells people that if your at stage 2, and it's the first day of the month, and you're \
            in area 15, then you'll have loadshedding from this time to that time. You probably won't \
            need this information for most use cases.\n\
            \n\
            ## About and Thank You's\n\
            \n\
            This project was written in [Rust](https://www.rust-lang.org/) by \
            [Boyd Kane](https://twitter.com/beyarkay), the backend is hosted by \
            [shuttle.rs](https://www.shuttle.rs/), the website you're looking is generated \
            automatically by [Swagger](https://swagger.io/tools/swagger-ui/) from the \
            [OpenAPI](https://www.openapis.org/) spec, which itself is generated from inline Rust \
            docstrings via [utoipa](https://github.com/juhaku/utoipa). \n\
        "
        ),
        paths(
            latest::outages,
            latest::fuzzy_search,
            latest::schedules,
            latest::list_all_areas,
            latest::list_areas,
        ),
        components(schemas(
            structs::Area,
            structs::AreaId,
            structs::ContiguousRegion,
            structs::Coords,
            structs::DistrictMunic,
            structs::LocalMunic,
            structs::MetroMunic,
            structs::Municipality,
            structs::PowerOutage,
            structs::Province,
            structs::Recurrence,
            structs::RecurringOutage,
            structs::RecurringSchedule,
            structs::ScheduleId,
            structs::SearchResult<structs::Area>,
        ))
    )]
    struct ApiDoc;

    let rocket = rocket::build()
        .attach(Cors)
        .mount("/", latest::routes())
        .mount("/v0.0.1", v0_0_1::routes())
        .mount(
            "/",
            SwaggerUi::new("/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );

    Ok(rocket.into())
}

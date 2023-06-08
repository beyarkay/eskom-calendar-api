use crate::{build_rocket, rocket};
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn non_empty_all_areas() {
    let client = Client::tracked(build_rocket()).expect("valid rocket instance");
    let response = client.get(uri!(crate::latest::list_all_areas)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let list_of_areas = response.into_json::<Vec<String>>().unwrap();
    assert!(
        !list_of_areas.is_empty(),
        "All areas length was {} which is not >0",
        list_of_areas.len()
    );
}

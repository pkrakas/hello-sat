use rocket::{ get, routes , launch};
use rocket::serde::json::{json, Value};
use rand::Rng;

#[allow(non_snake_case)]
#[get("/calculateDisselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn calculate_diesel_usage(distance: u32, yearOfProduction: u16, fuelUsagePer100KM: f32) -> Value {
    const CURRENT_YEAR: u16 = 2022;
    let elderness_ratio: f32 = ((CURRENT_YEAR - yearOfProduction) as f32 / 50.0) + 1.0;
    let fuel_usage: f32 = distance as f32 / 100.0 * fuelUsagePer100KM * elderness_ratio;
    json!({
        "fuelUsage": fuel_usage
    })
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[get("/probabilityOfUnitInjectorFail?<VIN>")]
fn probability_of_injector_fail(VIN: String) -> Value {
    let mut rng = rand::thread_rng();
    let randomNumber: f32 = rng.gen();
    json!({
        "failProbability": randomNumber
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![calculate_diesel_usage, probability_of_injector_fail])
}

use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sensor_data;
use crate::sensor_data::model::SensorData;
use crate::sensor_data::model::NewSensorData;

#[get("/sensor_data")]
pub fn all_sensor_data(connection: DbConn) -> Result<Json<Vec<SensorData>>, Status> {
    sensor_data::repository::show_sensor_data(&connection)
        .map(|data| Json(data))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_sensor_data>")]
pub fn create_sensor_data(new_sensor_data: Json<NewSensorData>, connection: DbConn) ->  Result<status::Created<Json<SensorData>>, Status> {
    println!("here 0 {}",&new_sensor_data.id);
    sensor_data::repository::create_sensor_data(new_sensor_data.into_inner(), &connection)
        .map(|data| sensor_data_created(data))
        .map_err(|error| error_status(error))

}

#[get("/<id>")]
pub fn get_sensor_data(id: i32, connection: DbConn) -> Result<Json<SensorData>, Status> {
    sensor_data::repository::get_sensor_data(id, &connection)
        .map(|data| Json(data))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<sensor_data>")]
pub fn update_sensor_data(id: i32, sensor_data: Json<SensorData>, connection: DbConn) -> Result<Json<SensorData>, Status> {
    sensor_data::repository::update_sensor_data(id, sensor_data.into_inner(), &connection)
        .map(|data| Json(data))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_sensor_data(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    sensor_data::repository::delete_sensor_data(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}


fn sensor_data_created(sensor_data: SensorData) -> status::Created<Json<SensorData>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/post/{id}", host = host(), port = port(), id = sensor_data.id).to_string(),
        Some(Json(sensor_data)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sensor_data::model::SensorData;
use crate::sensor_data::model::NewSensorData;

use crate::schema::sensor_data;
use crate::schema::sensor_data::dsl::*;

pub fn create_sensor_data(new_sensor_data: NewSensorData, conn: &PgConnection) -> QueryResult<SensorData> {
    diesel::insert_into(sensor_data::table)
        .values(&new_sensor_data)
        .get_result(conn)
}

pub fn show_sensor_data(connection: &PgConnection) -> QueryResult<Vec<SensorData>>  {
    //posts.filter(published.eq(true))
    sensor_data.limit(5)
        .load::<SensorData>(&*connection)
}

pub fn get_sensor_data(sensor_data_id: i32, connection: &PgConnection) -> QueryResult<SensorData> {
    sensor_data::table.find(sensor_data_id).get_result::<SensorData>(connection)
}

pub fn update_sensor_data(sensor_data_id: i32, data: SensorData, connection: &PgConnection) -> QueryResult<SensorData> {
    diesel::update(sensor_data::table.find(sensor_data_id))
        .set(&data)
        .get_result(connection)
}

pub fn delete_sensor_data(sensor_data_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(sensor_data::table.find(sensor_data_id))
        .execute(connection)
}

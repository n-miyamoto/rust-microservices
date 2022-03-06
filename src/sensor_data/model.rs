#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::sensor_data;


#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "sensor_data"]
pub struct SensorData { 
    pub id: i32,
    pub writekey: String, 
    pub create_at: chrono::NaiveDateTime,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="sensor_data"]
pub struct NewSensorData {
    pub create_at: chrono::NaiveDateTime,
    pub writekey: String, 
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ReceiveSensorData {
    pub writeKey: String, 
    pub d1: String,
    pub d2: String,
    pub d3: String,
}

#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::sensor_data;


#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "sensor_data"]
pub struct SensorData { 
    pub id: i32,
    pub create_at: chrono::NaiveDateTime,
    pub data0: Option<f32>,
    pub data1: Option<f32>,
    pub data2: Option<f32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="sensor_data"]
pub struct NewSensorData {
    pub id: i32,
    pub create_at : chrono::NaiveDateTime,
    pub data0: f32,
    pub data1: f32,
    pub data2: f32,
}

use rocket;

use crate::connection;
use crate::sensor_data;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/sensor_data",
               routes![
                    sensor_data::handler::all_sensor_data,
                    sensor_data::handler::create_sensor_data,
                    sensor_data::handler::get_sensor_data,
                    sensor_data::handler::update_sensor_data,
                    sensor_data::handler::delete_sensor_data,
                    ],
        ).launch();
}
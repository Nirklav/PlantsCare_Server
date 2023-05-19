#[macro_use]
extern crate log;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use hyper::Server;
use hyper::service::{make_service_fn, service_fn};

use server::RpiHomeContext;
use config::Config;
use utils::camera::Camera;

use requests::*;
use crate::utils::water_sensor::WaterSensor;
use crate::utils::water_pump::WaterPump;
use crate::utils::servo::Servo;
use crate::services::climate::Climate;
use crate::services::switches::Switches;

mod config;
mod server;
mod requests;
mod utils;
mod services;
mod commands;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    args.next(); // skip exe
    let config_path = match args.next() { // path to config
        Some(c) => c,
        None => panic!("first argument should be path to config file")
    };

    let config = match Config::from_file(&config_path) {
        Ok(c) => c,
        Err(e) => panic!("error on config read: {}", e)
    };

    if let Err(e) = log4rs::init_file(&config.log_config_path, Default::default()) {
        panic!("error on logger init: {}", e);
    }

    let socket_addr = match SocketAddr::from_str(&config.address) {
        Ok(a) => a,
        Err(e) => panic!("error on address parse {}", e)
    };

    let camera = match Camera::new() {
        Ok(c) => Arc::new(c),
        Err(e) => panic!("error on camera creation {}", e)
    };

    let water_sensor = match WaterSensor::new() {
        Ok(ws) => Arc::new(ws),
        Err(e) => panic!("error on water sensor creation {}", e)
    };

    let water_pump = match WaterPump::new() {
        Ok(wp) => Arc::new(wp),
        Err(e) => panic!("error on water pump creation {}", e)
    };

    let servo = match Servo::new() {
        Ok(s) => Arc::new(s),
        Err(e) => panic!("error on servo creation {}", e)
    };

    let climate = Arc::new(Climate::new());
    let switches = Arc::new(Switches::new());

    let mut context = RpiHomeContext::new();
    context.add_handler(echo_request::EchoRequest::new());

    context.add_handler(get_camera_image_request::GetCameraImageRequest::new(&config.protected_key, &camera));
    context.add_handler(is_enough_water_request::IsEnoughWaterRequest::new(&config.protected_key, &water_sensor));
    context.add_handler(water_request::WaterRequest::new(&config.protected_key, &water_sensor, &water_pump));
    context.add_handler(turn_servo_request::TurnServoRequest::new(&config.protected_key, &servo));

    context.add_handler(conditioners_request::ConditionersRequest::new(&config.protected_key, &climate));
    context.add_handler(get_climate_request::GetClimateRequest::new(&config.protected_key, &climate));
    context.add_handler(set_climate_request::SetClimateRequest::new(&config.protected_key, &climate));

    context.add_handler(is_enabled_request::IsEnabledRequest::new(&config.protected_key, &switches));
    context.add_handler(set_switch_request::SwitchRequest::new(&config.protected_key, &switches));

    let context = Arc::new(context);

    let make_service = make_service_fn(move |_conn| {
        let context = context.clone();
        let service = service_fn(move |req| {
            RpiHomeContext::handle(context.clone(), req)
        });

        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&socket_addr).serve(make_service);

    println!("Listening on http://{}", socket_addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
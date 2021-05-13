extern crate futures;
extern crate hyper;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate log4rs;

use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use hyper::server::{Http, Request, Response, NewService};

use server::BucketService;
use config::Config;
use utils::camera::Camera;

use requests::*;
use crate::utils::water_sensor::WaterSensor;

mod config;
mod server;
mod requests;
mod utils;

fn main() {
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
        Ok(c) => c,
        Err(e) => panic!("error on camera creation {}", e)
    };

    let water_sensor = match WaterSensor::new() {
        Ok(ws) => ws,
        Err(e) => panic!("error on water sensor creation {}", e)
    };

    let factory = PlantsCareServiceFactory::new(config.protected_key.clone(), camera, water_sensor);
    let server = match Http::new().bind(&socket_addr, factory) {
        Ok(s) => s,
        Err(e) => panic!("error on server bind: {}", e)
    };

    info!("Running server...");

    if let Err(e) = server.run() {
        panic!("error on server run: {}", e)
    };
}

struct PlantsCareServiceFactory {
    protected_key: String,
    camera: Arc<Camera>,
    water_sensor: Arc<WaterSensor>
}

impl PlantsCareServiceFactory {
    fn new(protected_key: String, camera: Camera, water_sensor: WaterSensor) -> Self {
        PlantsCareServiceFactory {
            protected_key,
            camera: Arc::new(camera),
            water_sensor: Arc::new(water_sensor)
        }
    }
}

impl NewService for PlantsCareServiceFactory {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = BucketService;

    fn new_service(&self) -> std::io::Result<Self::Instance> {
        let mut service = BucketService::new();
        service.add_handler(echo_request::EchoRequest::new());
        service.add_handler(get_camera_image_request::GetCameraImageRequest::new(&self.protected_key, &self.camera));
        service.add_handler(is_enough_water_request::IsEnoughWaterRequest::new(&self.protected_key, &self.water_sensor));
        Ok(service)
    }
}
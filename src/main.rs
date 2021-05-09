use std::{thread, time};
use std::time::Duration;

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

    let server = match Http::new().bind(&socket_addr, BucketServiceFactory::new(camera)) {
        Ok(s) => s,
        Err(e) => panic!("error on server bind: {}", e)
    };

    println!("Running server...");

    if let Err(e) = server.run() {
        panic!("error on server run: {}", e)
    };
}

struct BucketServiceFactory {
    camera: Arc<Camera>
}

impl BucketServiceFactory {
    fn new(camera: Camera) -> Self {
        BucketServiceFactory {
            camera: Arc::new(camera)
        }
    }
}

impl NewService for BucketServiceFactory {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = BucketService;

    fn new_service(&self) -> std::io::Result<Self::Instance> {
        let mut service = BucketService::new();
        service.add_handler(echo_request::EchoRequest::new());
        service.add_handler(get_camera_image_request::GetCameraImageRequest::new(&self.camera));
        Ok(service)
    }
}
extern crate argparse;
extern crate consul;
extern crate iron;
#[macro_use]
extern crate log;
extern crate log4rs;

mod discovery;

use argparse::{ ArgumentParser, Store };
use iron::prelude::*;
use iron::status;
use std::thread;

fn configure_logging() {
    log4rs::init_file("conf/log4rs.yml", Default::default()).unwrap();
}

fn parse_args() -> u16 {
    let mut port = 8000;

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Rusty Web Service");
        parser.refer(&mut port).add_option(&["-p", "--port"], Store,
            "Port to bind to.");

        parser.parse_args_or_exit();
    }

    port
}

fn register_consul(port: u16) -> thread::JoinHandle<()> {
    thread::Builder::new().name("consul".to_string()).spawn(move || {
        discovery::register("127.0.0.1".to_string(), port).unwrap();
        info!("Registered with Consul on Port {}", port);
    }).unwrap() // returns a Result<JoinHandle<T>>
}

fn launch(port: u16) {
    info!("Launching Iron...");
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "{ message: \"Success!\" }")))
    }).http(format!("127.0.0.1:{}", port)).unwrap();
}

fn main() {
    configure_logging();

    let port = parse_args();

    register_consul(port).join().unwrap();

    launch(port)
}

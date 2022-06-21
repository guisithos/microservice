use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App};
use dotenv::dotenv;
use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use log::{debug, info, trace};
use std::env;

fn main() {
    dotenv().ok();
    env_logger::init();

    //App instance is fed with arg instances. It has also methods
    //used to set infos about the app.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
	//with_name method add arguments and a chaining call.
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("address")
             .short("a")
             .long("address")
             .value_name("address")
             .help("Sets an address")
             .takes_value(true))
        .get_matches();
    info!("Rand Microservice - 0.1");
    trace!("Starting...");

    //Reads the address value, if theres none, code won't throw a panic.
    let addr = matches.value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("address").ok())
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .parse()
        .expect("can't parse address variable");
    debug!("Trying to bind server to addr: {}", addr);

    let builder = Server::bind(&addr);
    trace!("Creating service handler");

    let server = builder.serve(|| {
        service_fn_ok(|req| {
            trace!("Incoming request is: {:?}", req);

            let random_byte = rand::random::<u8>();
            debug!("Generated value is: {}", random_byte);
            Response::new(Body::from(random_byte.to_string()))
        })
    });
    info!("Used address: {}", server.local_addr());

    let server = server.map_err(drop);
    debug!("RuN");
    hyper::rt::run(server);
}

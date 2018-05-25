use ansi_term::Colour;
use clap;
use env_logger::Builder;
use errors::Result;
use log::LevelFilter;
use std::env;
use actors::Signals;
use actix::prelude::*;
use misc::version;
use actors::GraphemeCounter;
use actix_web::{middleware, http, server, App, http::header, middleware::cors::Cors};
use handlers::{self, State};
use dirs::Directories;

fn init_logger(pattern: &str) {
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");
    let mut builder = Builder::new();
    // Disable info logging by default for some modules:
    builder.filter(Some("hyper"), LevelFilter::Warn);
    builder.filter(Some("mio::timer"), LevelFilter::Warn);
    builder.filter(Some("mio::poll"), LevelFilter::Warn);
    builder.filter(Some("tokio_reactor"), LevelFilter::Warn);
    builder.filter(Some("tokio_core"), LevelFilter::Warn);
    builder.filter(Some("tokio_threadpool"), LevelFilter::Warn);
    builder.filter(Some("actix_web::middleware::logger"), LevelFilter::Info);
    builder.filter(Some("actix_web::server::h1decoder"), LevelFilter::Warn);
    builder.filter(Some("trust_dns_proto"), LevelFilter::Warn);

    // Enable info for others.
    builder.filter(None, LevelFilter::Info);

    if let Ok(lvl) = env::var("RUST_LOG") {
        builder.parse(&lvl);
    }

    builder.parse(pattern);
    builder.init();
}

fn get_api_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    port_str.parse().unwrap_or(8080)
}

fn interface(interface: &str) -> String {
    match interface {
        "all" => "0.0.0.0",
        "local" => "127.0.0.1",
        x => x,
    }.into()
}

pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<::std::ffi::OsString> + Clone,
{
    let yaml = load_yaml!("./cli.yml");
    let matches = clap::App::from_yaml(yaml)
        .version(version().as_ref())
        .get_matches_from_safe(args)?;
    let log_pattern = matches.value_of("log").unwrap_or("");
    init_logger(log_pattern);

    let port = value_t!(matches, "port", u16).unwrap_or(get_api_port());
    let inet = interface(matches.value_of("interface").unwrap());    
    let base = matches.value_of("base");

    let mut dirs = Directories::default();
    if let Some(dbpath) = base {
        dirs.base = String::from(dbpath);
    }    
    let _ = dirs.create_dirs();

    info!("Starting {}", Colour::White.bold().paint(version()));
    let sys = System::new("hello");  

    // Start signals handler
    let _addr: Addr<Syn, _> = Signals.start(); 

    let counter_addr = SyncArbiter::start(::num_cpus::get(), move || {
        GraphemeCounter{}
    }); 


    server::new(move || {            
        App::with_state(State{counter_addr: counter_addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .configure(|app| Cors::for_app(app)
                .allowed_methods(vec!["GET", "POST", "DELETE"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)            
                .resource("/", |r| r.method(http::Method::GET).with2(handlers::count))
                .register())
        })
        .bind((inet.as_str(), port)).unwrap()
        .shutdown_timeout(1)
        .start();        
     

    let code = sys.run();
    ::std::process::exit(code);
}

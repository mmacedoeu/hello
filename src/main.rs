extern crate app_dirs;
extern crate env_logger;
extern crate actix;
extern crate ansi_term;
extern crate target_info;
extern crate unicode_segmentation;
extern crate futures;
extern crate actix_web;
extern crate num_cpus;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod errors;
mod dirs;
mod misc;
mod handlers;
mod actors;
mod cli;

quick_main!(run);

fn run() -> errors::Result<()> {
	cli::run(::std::env::args())
}

use actix::actors::signal;
use actix::prelude::*;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use errors::Result;
use handlers::CountParam;

pub struct Signals;

impl Actor for Signals {
    type Context = Context<Self>;
}

// Shutdown system on and of `SIGINT`, `SIGTERM`, `SIGQUIT` signals
impl Handler<signal::Signal> for Signals {
    type Result = ();

    fn handle(&mut self, msg: signal::Signal, _: &mut Context<Self>) {
        match msg.0 {
            signal::SignalType::Int => {
                println!("SIGINT received, exiting");
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
            }
            signal::SignalType::Hup => {
                println!("SIGHUP received, reloading");
            }
            signal::SignalType::Term => {
                println!("SIGTERM received, stopping");
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
            }
            signal::SignalType::Quit => {
                println!("SIGQUIT received, exiting");
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
            }
            _ => (),
        }
    }
}

pub fn count(input: &str) -> Vec<(String, u64)>{
    let mut map = HashMap::new();
    UnicodeSegmentation::graphemes(input, true)
        .for_each(|c| {
            map.entry(c)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
        });
    let mut out : Vec<(String, u64)> = map.iter().map(|a| ((*a.0).into(),(*a.1))).collect();
    out.sort_unstable_by(|a,b| b.1.cmp(&a.1));
    out
}

pub struct GraphemeCounter;

impl Actor for GraphemeCounter {
    type Context = SyncContext<Self>;
}

// pub struct CounterInput(pub String);

impl Message for CountParam {
    type Result = Result<Vec<(String, u64)>>;
}

impl Handler<CountParam> for GraphemeCounter {
    type Result = Result<Vec<(String, u64)>>;

    fn handle(&mut self, msg: CountParam, _: &mut Self::Context) -> Self::Result {
        Ok(count(&msg.input))
    }
}


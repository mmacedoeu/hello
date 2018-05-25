use actix::{Addr, Syn};
use actix_web::{Error, HttpRequest, HttpResponse, Query};
use actors::GraphemeCounter;
use futures::Future;

pub struct State {
    pub counter_addr: Addr<Syn, GraphemeCounter>,
}

#[derive(Deserialize, Clone)]
pub struct CountParam {
    pub input: String,
}

pub fn count(
    req: HttpRequest<State>,
    info: Query<CountParam>,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    Box::new(
        req.state()
            .counter_addr
            .send(info.clone())
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(out) => Ok(HttpResponse::Ok().json(out)), // <- send response
                    Err(e) => {
                        warn!("error: {:?}", e);
                        Ok(HttpResponse::InternalServerError().into())
                    }
                }
            })
    )
}

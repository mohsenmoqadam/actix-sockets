use crate::lobby::Lobby;
use crate::ws::WsConn;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<uuid::Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let group_id = path.into_inner();
    let ws = WsConn::new(group_id, srv.get_ref().clone());

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

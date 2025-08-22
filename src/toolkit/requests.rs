use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::client::conn::http1;
use hyper::{Request, Response, Uri};
use hyper_util::rt::TokioIo;
use native_tls::TlsConnector;
use serde::{Deserialize, Serialize};
use std::io;
use std::time::SystemTime;
use tokio::net::TcpStream;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseJson {
    pub status_code: u16,
    pub body: String,
    // pub timestamp: u128,
}

pub async fn request_http1_tls_response(req: Request<Full<Bytes>>) -> Result<ResponseJson, String> {
    let (host, authority) = match get_host_and_authority(&req.uri()) {
        Some(stream) => stream,
        _ => return Err("failed to get authority from uri".to_string()),
    };

    let io = match create_tls_stream(&host, &authority).await {
        Ok(stream) => stream,
        Err(e) => return Err(e),
    };

    let (mut sender, conn) = match http1::handshake(io).await {
        Ok(handshake) => handshake,
        Err(e) => return Err(e.to_string()),
    };

    tokio::task::spawn(async move {
        if let Err(_err) = conn.await { /* log connection error */ }
    });

    let res = match sender.send_request(req).await {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };

    convert_response_to_json_struct(res).await
}

pub fn get_host_and_authority(uri: &Uri) -> Option<(&str, String)> {
    let scheme = match uri.scheme() {
        Some(s) => s.as_str(),
        _ => hyper::http::uri::Scheme::HTTPS.as_str(),
    };

    let port = match (uri.port(), scheme) {
        (Some(p), _) => p.as_u16(),
        (None, "https") => 443,
        _ => 80,
    };

    let host = match uri.host() {
        Some(h) => h,
        _ => return None,
    };

    let authority = host.to_string() + ":" + &port.to_string();

    Some((host, authority))
}

// this has multiple "types" of errors
// signal that it is an inappropriate grouping?
async fn create_tls_stream(
    host: &str,
    addr: &str,
) -> Result<TokioIo<tokio_native_tls::TlsStream<TcpStream>>, String> {
    let tls_connector = match TlsConnector::new() {
        Ok(cx) => tokio_native_tls::TlsConnector::from(cx),
        Err(e) => return Err(e.to_string()),
    };

    let client_stream = match TcpStream::connect(addr).await {
        Ok(s) => s,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let tls_stream = match tls_connector.connect(host, client_stream).await {
        Ok(s) => TokioIo::new(s),
        Err(e) => return Err(e.to_string()),
    };

    Ok(tls_stream)
}

async fn convert_response_to_json_struct(res: Response<Incoming>) -> Result<ResponseJson, String> {
    // let timestamp = match get_timestamp() {
    //     Ok(n) => n,
    //     Err(e) => return Err(e),
    // };

    let status = res.status().as_u16();

    let body_str = match response_body_to_string(res).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    Ok(ResponseJson {
        status_code: status,
        body: body_str,
        // timestamp: timestamp,
    })
}

async fn response_body_to_string(response: Response<Incoming>) -> Result<String, String> {
    // asynchronously aggregate the chunks of the body
    let body = match response.collect().await {
        Ok(b) => b.aggregate(),
        Err(e) => return Err(e.to_string()),
    };

    let ip_str = match io::read_to_string(body.reader()) {
        Ok(b) => b,
        Err(e) => return Err(e.to_string()),
    };

    Ok(ip_str.to_string())
}

// fn get_timestamp() -> Result<u128, String> {
//     match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//         Ok(n) => Ok(n.as_millis()),
//         Err(e) => Err(e.to_string()),
//     }
// }

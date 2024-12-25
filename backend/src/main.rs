use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    body, Body, Method, Request, Response, Server,
    Client,
};
use hyper::service::{make_service_fn, service_fn};
use hyper_tls::HttpsConnector;
use uuid::Uuid;

mod db;
use db::models;
use db::schema;
mod proxy;

use crate::models::{CreateRequest, CreateResponse};

#[tokio::main(flavor = "multi_thread", worker_threads = 15)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let base_domain = "directdns.net";
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Reverse proxy listening on http://{} ...", addr);

    // Create an HTTPS client
    let https_connector = HttpsConnector::new();
    let client: Client<_, Body> = Client::builder().build(https_connector);

    // Build the hyper server
    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        let base_domain = base_domain.to_string();

        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let client = client.clone();
                let base_domain = base_domain.clone();

                async move {
                    // Handle /api/create separately
                    if req.uri().path() == "/api/create" {
                        return handle_create(req).await;
                    }
                    // Otherwise, handle proxy
                    proxy::handle_proxy_request(req, &client, &base_domain)
                        .await
                        .or_else(|err| {
                            eprintln!("Proxy error: {}", err);
                            let resp = Response::builder()
                                .status(500)
                                .body(Body::from("Internal Server Error"))
                                .unwrap();
                            Ok(resp)
                        })
                }
            }))
        }
    });

    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}

/// Handle the POST /api/create request
async fn handle_create(
    mut req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    // If preflight
    if req.method() == Method::OPTIONS {
        return Ok(cors_response(Response::new(Body::empty())));
    }

    if req.method() != Method::POST {
        return Ok(Response::builder()
            .status(405)
            .body(Body::from("Method not allowed"))
            .unwrap());
    }

    // Read the entire request body
    let whole_body = match body::to_bytes(req.body_mut()).await {
        Ok(b) => b,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Invalid body"))
                .unwrap());
        }
    };

    // Deserialize JSON
    let create_req: Result<CreateRequest, _> = serde_json::from_slice(&whole_body);
    let create_req = match create_req {
        Ok(cr) => cr,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Invalid JSON"))
                .unwrap());
        }
    };

    // Generate a short random token
    let token = Uuid::new_v4().to_string()[..8].to_owned();

    // Insert into DB
    let conn = &mut db::establish_connection();

    if let Err(e) = db::insert_session(conn, &token, &create_req.hostname, create_req.ip_address.as_deref()) {
        eprintln!("DB insert error: {}", e);
        return Ok(Response::builder()
            .status(500)
            .body(Body::from("Internal DB Error"))
            .unwrap());
    }
    let base_domain = "directdns.net";

    let full_url = format!("{}.{}", token, base_domain);

    // Build JSON response
    let response_body = serde_json::to_string(&CreateResponse { 
        token,
        full_url,
        domain: base_domain.to_string(),
    }).unwrap();
    let mut resp = Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .body(Body::from(response_body))
        .unwrap();

    cors_headers(&mut resp);
    Ok(resp)
}

/// Convenience function to add CORS headers to a response.
fn cors_response<T>(mut resp: Response<T>) -> Response<T> {
    let headers = resp.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "POST, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());
    resp
}

fn cors_headers<T>(resp: &mut Response<T>) {
    let headers = resp.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "POST, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());
}

use hyper::{
    Body,
    client::{HttpConnector},
    header::HOST,
    Request, Response,
};
use hyper_tls::HttpsConnector;
use std::error::Error;

use crate::db;

/// Strips any trailing port from the given host string.
/// E.g. "example.com:1234" -> "example.com"
fn strip_port(host_with_port: &str) -> &str {
    if let Some(idx) = host_with_port.rfind(':') {
        let after_colon = &host_with_port[idx + 1..];
        if after_colon.chars().all(|c| c.is_ascii_digit()) {
            return &host_with_port[..idx];
        }
    }
    host_with_port
}

/// Extract the subdomain from something like "<token>.somedomain.com"
fn extract_subdomain(host_header: &str, base_domain: &str) -> Option<String> {
    if host_header == base_domain {
        return Some("".to_string());
    }

    let dot_and_domain = format!(".{}", base_domain);
    if host_header.ends_with(&dot_and_domain) {
        let subdomain_part = &host_header[..host_header.len() - dot_and_domain.len()];
        return Some(subdomain_part.to_string());
    }
    None
}

/// Handles requests that are not /api routes.
/// This looks up the session by `token` and then proxies to either IP (HTTP) or hostname (HTTPS).
pub async fn handle_proxy_request(
    mut req: Request<Body>,
    client: &hyper::Client<HttpsConnector<HttpConnector>, Body>,
    base_domain: &str,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    // Grab host from the Host header
    let host_header = req
        .headers()
        .get(HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let domain_only = strip_port(host_header);

    // subdomain = the token
    let token = match extract_subdomain(domain_only, base_domain) {
        Some(x) if !x.is_empty() => x,
        _ => {
            // If no subdomain or it doesn't match, 404
            return Ok(Response::builder()
                .status(404)
                .body(Body::from("Not found"))?);
        }
    };

    // Lookup the session
    let conn = &mut db::establish_connection();

    let maybe_session = db::get_session_by_token(conn, &token)?;

    let session = match maybe_session {
        Some(s) => s,
        None => {
            return Ok(Response::builder()
                .status(404)
                .body(Body::from("Not found"))?);
        }
    };

    // Rewrite the request URI
    let old_uri = req.uri().clone();
    let path_and_query = old_uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("");

    // If session.ip_address is Some(...), proxy over HTTP by IP
    // Otherwise, proxy over HTTPS by hostname
    let new_uri_str = if let Some(ip) = &session.ip_address {
        format!("http://{}{}", ip, path_and_query)
    } else {
        if !(session.hostname.starts_with("http://") || session.hostname.starts_with("https://")) {
            format!("https://{}{}", session.hostname, path_and_query)
        } else {
            format!("{}{}", session.hostname, path_and_query)
        }
    };
    let new_uri = new_uri_str.parse()?;
    *req.uri_mut() = new_uri;

    // Remove Host header so that hyper sets it based on the new URI,
    // or we can explicitly set it to session.hostname.
    req.headers_mut().remove(HOST);
    if session.ip_address.is_some() {
        req.headers_mut()
            .insert(HOST, session.hostname.parse().unwrap());
    }

    // Forward the request
    let resp = client.request(req).await?;
    Ok(resp)
}

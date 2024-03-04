use axum::http::HeaderMap;

pub mod config;
pub mod database;
pub mod middleware;
pub mod router;

pub mod user;
pub mod resident;
pub mod security;


pub fn sanitize_headers(headers: HeaderMap, key: &str) -> Result<String, String>{
    let header_val = match headers.get(key) {
        Some(header) => header,
        None => {
            return Err(format!("Expected header: {} was not provided", key));
        }
    };

    return match header_val.to_str() {
        Ok(header_str) => Ok(String::from(header_str)),
        Err(err) => Err(err.to_string())
    };
}
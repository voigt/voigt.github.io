use anyhow::Result;
use http::{Method, StatusCode};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    key_value::{Error, Store},
};
use std::str;

use serde::Serialize;
#[derive(Serialize, Debug)]
struct Article {
    id: String,
    count: i32,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> Result<Response> {
    println!(
        "{:?} - {:?} - {:?}",
        req.uri().path(),
        req.method(),
        req.headers()
    );
    // Open the default key-value store
    let store = Store::open_default()?;

    let (status, body) = match req.method() {
        &Method::OPTIONS => (StatusCode::NO_CONTENT, None),
        &Method::GET => {
            // Get the value associated with the request URI, or return a 404 if it's not present
            match store.get(req.uri().path()) {
                Ok(value) => {
                    let s = match str::from_utf8(&value) {
                        Ok(v) => v,
                        Err(e) => return Err(e.into()),
                    };

                    let article = Article {
                        id: req.uri().path().into(),
                        count: s.parse::<i32>().unwrap(),
                    };

                    let json = serde_json::to_string(&article).unwrap();
                    (StatusCode::OK, Some(json.into()))
                }
                Err(Error::NoSuchKey) => (StatusCode::NOT_FOUND, None),
                Err(error) => return Err(error.into()),
            }
        }
        &http::Method::PATCH => {
            match store.get(req.uri().path()) {
                Ok(value) => {
                    let s = match str::from_utf8(&value) {
                        Ok(v) => v,
                        Err(e) => return Err(e.into()),
                    };

                    let count = match s.parse::<i32>() {
                        Ok(c) => c,
                        Err(e) => return Err(e.into()),
                    };

                    let mut article = Article {
                        id: req.uri().path().into(),
                        count: 0,
                    };

                    if count > 0 {
                        article.count = count - 1;
                    }

                    // Key exists, so we want to increment it
                    // Add the request (URI, body) tuple to the store
                    // store.set(req.uri().path(), count.to_string())?;
                    match store.set(req.uri().path(), article.count.to_string()) {
                        Ok(ok) => ok,
                        Err(e) => return Err(e.into()),
                    }

                    let json = serde_json::to_string(&article).unwrap();

                    (StatusCode::OK, Some(json.into()))
                },
                Err(Error::NoSuchKey) => {
                    let article = Article {
                        id: "does-not-exist".into(),
                        count: 0,
                    };

                    let json = serde_json::to_string(&article).unwrap();

                    (StatusCode::OK, Some(json.into()))
                }
                Err(error) => return Err(error.into()),
            }
        },
        &Method::POST => {
            match store.get(req.uri().path()) {
                Ok(value) => {
                    let s = match str::from_utf8(&value) {
                        Ok(v) => v,
                        Err(e) => return Err(e.into()),
                    };
                    
                    let count = match s.parse::<i32>() {
                        Ok(c) => c,
                        Err(e) => return Err(e.into()),
                    };

                    let article = Article {
                        id: req.uri().path().into(),
                        count: count + 1,
                    };
                    
                    // Key exists, so we want to increment it
                    // Add the request (URI, body) tuple to the store
                    // store.set(req.uri().path(), count.to_string())?;
                    match store.set(req.uri().path(), article.count.to_string()) {
                        Ok(ok) => ok,
                        Err(e) => return Err(e.into()),
                    }
                    
                    let json = serde_json::to_string(&article).unwrap();
                    
                    (StatusCode::OK, Some(json.into()))
                }
                Err(Error::NoSuchKey) => {
                    // If no key, create it (if pattern matches?)!
                    match store.set(req.uri().path(), "1") {
                        Ok(ok) => ok,
                        Err(e) => return Err(e.into()),
                    }
                    
                    let article = Article {
                        id: "".into(),
                        count: 1,
                    };
                    
                    let json = serde_json::to_string(&article).unwrap();

                    // (StatusCode::OK, Some(json))
                    (StatusCode::OK, Some(json.into()))
                }
                Err(error) => return Err(error.into()),
            }
        },
        _ => (StatusCode::METHOD_NOT_ALLOWED, None),
    };
    
    Ok(http::Response::builder()
    .status(status)
    .header("Content-Type", "application/json")
    .header("Access-Control-Allow-Origin", "https://cvo.fermyon.app")
    .header("Access-Control-Allow-Methods", "*")
    .header("Access-Control-Allow-Headers", "*")
    .header("Access-Control-Max-Age", "86400")
    .body(body)?)
}

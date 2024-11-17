use spin_sdk::{
    http::{IntoResponse, Request, Method, },
    http_component,
    key_value::Store,
};

use core::str;

use serde::Serialize;
#[derive(Serialize, Debug)]
struct Article {
    id: String,
    count: i32,
}

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {

    // Open the default key-value store
    let store = Store::open_default()?;

    let (status, body) = match *req.method() {
        Method::Options => (204, None),
        Method::Post => {
            match store.get(req.path())? {
                Some(value) => {
                    let s = match str::from_utf8(&value) {
                        Ok(v) => v,
                        Err(e) => return Err(e.into()),
                    };
                    
                    let count = match s.parse::<i32>() {
                        Ok(c) => c,
                        Err(e) => return Err(e.into()),
                    };

                    let article = Article {
                        id: req.path().to_string(),
                        count: count + 1,
                    };

                    // Key exists, so we want to increment the count
                    // Add the request (URI, body) tuple to the store
                    // store.set(req.path(), &article.count.to_be_bytes())?;
                    match store.set(req.path(), &article.count.to_string().as_bytes()){
                        Ok(_) => println!("Stored value in the KV store with {:?} as the key", req.path()),
                        Err(e) => return Err(e.into()),
                    }

                    let json = serde_json::to_string(&article).unwrap();

                    (200, Some::<String>(json.into()))
                }
                None => {
                    match store.set(req.path(), "1".as_bytes()) {
                        Ok(_) => println!("Stored value in the KV store with {:?} as the key", req.path()),
                        Err(e) => return Err(e.into()),
                    }

                    let article = Article {
                        id: "".into(),
                        count: 1,
                    };

                    let json = serde_json::to_string(&article).unwrap();

                    (200, Some::<String>(json.into()))
                }
            }
        }
        Method::Get => {
            // Get the value associated with the request URI, or return a 404 if it's not present
            match store.get(req.path())? {
                Some(value) => {
                    let s = match str::from_utf8(&value) {
                        Ok(v) => v,
                        Err(e) => return Err(e.into()),
                    };

                    let article = Article {
                        id: req.path().into(),
                        count: s.parse::<i32>().unwrap(),
                    };

                    let json = serde_json::to_string(&article).unwrap();
                    
                    (200, Some(json.into()))
                }
                None => {
                    println!("No value found for the key {:?}", req.path());
                    let article = Article {
                        id: req.path().into(),
                        count: 0,
                    };

                    let json = serde_json::to_string(&article).unwrap();
                    
                    (200, Some(json.into()))
                }
            }
        }
        Method::Patch => {
            match store.get(req.path())? {
                Some(value) => {
                    let s = match str::from_utf8(&value) {
                        Ok(v) => v,
                        Err(e) => return Err(e.into()),
                    };
                    
                    let count = match s.parse::<i32>() {
                        Ok(c) => c,
                        Err(e) => return Err(e.into()),
                    };

                    let mut article = Article {
                        id: req.path().to_string(),
                        count: 0,
                    };

                    if count > 0 {
                        article.count = count - 1;
                    }

                    // Key exists, so we want to increment the count
                    // Add the request (URI, body) tuple to the store
                    // store.set(req.path(), &article.count.to_be_bytes())?;
                    match store.set(req.path(), &article.count.to_string().as_bytes()){
                        Ok(_) => println!("Stored value in the KV store with {:?} as the key", req.path()),
                        Err(e) => return Err(e.into()),
                    }

                    let json = serde_json::to_string(&article).unwrap();

                    (200, Some::<String>(json.into()))
                }
                None => {
                    let article = Article {
                        id: "does-not-exist".into(),
                        count: 0,
                    };

                    let json = serde_json::to_string(&article).unwrap();

                    (200, Some::<String>(json.into()))
                }
            }
        }
        // No other methods are currently supported
        _ => (405, None),
    };
    Ok(http::Response::builder()
    .status(status)
    .header("Content-Type", "application/json")
    .header("Access-Control-Allow-Origin", "https://christophvoigt.com")
    .header("Access-Control-Allow-Origin", "http://127.0.0.1:3000")
    .header("Access-Control-Allow-Origin", "http://cvo.fermyon.app")
    .header("Access-Control-Allow-Methods", "*")
    .header("Access-Control-Allow-Headers", "*")
    .header("Access-Control-Max-Age", "86400")
    .body(body)?)
}

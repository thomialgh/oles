
use std::collections::HashMap;
use hyper::{Request, Body};

// pub type HandleFunc = fn(Request<Body>) -> Result<Response<Body>, Infallible>;

pub struct Query {
    query_map: HashMap<String, String>
}

impl Query {
    pub fn new(query_string: Option<&str>) -> Self {
        let mut query_map = HashMap::new();
        if let Some(query) = query_string {
            query.split("&")
                .for_each(|i| {
                    let item: Vec<&str> = i.split("=").collect();
                    if item.len() == 2 {
                        query_map.insert(item[0].to_string(), item[1].to_string());
                    }
                })
        }

        Self {query_map}
    }

    pub fn get_item(&self, key: String) -> Option<&String> {
        self.query_map.get(&key)
    }
}

pub struct Params {
    params_map: HashMap<String, String>
}

impl Params {
    pub fn new() -> Self {
        Self {
            params_map: HashMap::new()
        }
    }

    pub fn insert(&mut self, k: String, v: String) {
        self.params_map.insert(k, v);
    }

    pub fn get_item(&self, key: String) -> Option<&String> {
        self.params_map.get(&key)
    }
}


pub struct Context {
    pub query: Query,
    pub params: Params,
    pub req: Request<Body>
}

impl Context {
    pub fn new(query: Query, params: Params, req: Request<Body>) -> Self {
        Self {query, params, req}
    }
}
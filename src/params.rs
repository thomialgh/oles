
use std::{convert::Infallible, collections::HashMap};
use hyper::{Request, Body, Response};

// pub type HandleFunc = fn(Request<Body>) -> Result<Response<Body>, Infallible>;
pub type HandleFunc = fn(ContextHandler, Request<Body>) -> Result<Response<Body>, Infallible>;


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



#[derive(Debug)]
pub struct QueryParams {
    query_map: HashMap<String, String>
}

impl QueryParams {
    pub fn new(req: &Request<Body>) -> Self {
        let mut query_map: HashMap<String, String> = HashMap::new();

        if let Some(query) = req.uri().query() {
            query.split("&").for_each(|qw| {
                let item: Vec<&str> = qw.split("=").collect();
                if item.len() == 2 {
                    query_map.insert(item[0].to_string(), item[1].to_string());
                }
            })
        }
        Self { query_map }
    }
    pub fn get(&self, key: &str) -> String {
        if let Some(val) = self.query_map.get(key) {
            String::from(val)
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct PathParams {
    path_map: HashMap<String, String>
}

impl PathParams {
    pub fn new() -> Self {
        Self {path_map : HashMap::new()}
    } 

    pub fn insert(&mut self, key: &str, value: &str) {
        self.path_map.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> String {
        if let Some(val) = self.path_map.get(key) {
            String::from(val)
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct ContextHandler {
    pub params: PathParams,
    pub query: QueryParams,
    pub req: Request<Body>,
}

impl ContextHandler {
    pub fn new(params: PathParams, query: QueryParams, req: Request<Body>) -> Self {
        Self {params, query, req}
    }

    pub async fn json_body<T>(&mut self) -> Result<T, Box<dyn std::error::Error>> 
    where 
        T: serde::de::DeserializeOwned
    {
        let body = hyper::body::to_bytes(self.req.body_mut()).await?;
        Ok(serde_json::from_slice(&body)?)
    }
}
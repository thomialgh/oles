
use std::{convert::Infallible, collections::HashMap};
use hyper::{Request, Body, Response};

// pub type HandleFunc = fn(Request<Body>) -> Result<Response<Body>, Infallible>;
pub type HandleFunc = fn(ContextHandler, Request<Body>) -> Result<Response<Body>, Infallible>;


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
    pub query: QueryParams
}

impl ContextHandler {
    pub fn new(params: PathParams, query: QueryParams) -> Self {
        Self {params, query}
    }
}
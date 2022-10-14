use std::{future::Future, collections::HashMap, sync::Arc, convert::Infallible};

use async_trait::async_trait;
use hyper::{Method, Request, Body};
use regex::Regex;
use crate::{
    params::{Context, Params, Query, self},
    response::{Resp, IntoResponse},
    service::Service

};

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self, svc: Arc<Service>, ctx: Context) -> Resp;
}

#[async_trait]
impl <F: Send + Sync + 'static, Fut> Handler for F 
where
    F: Fn(Arc<Service>, Context) -> Fut,
    Fut: Future<Output = Resp> + Send + 'static
{
    async fn invoke(&self, svc: Arc<Service>, ctx: Context) -> Resp {
        (self)(svc, ctx).await
    }
}

pub struct RouterHandler {
    regex: Regex,
    handler: Box<dyn Handler>
}

impl RouterHandler {
    pub fn builder(path: String, handler: Box<dyn Handler>) -> Result<Self, Box<dyn std::error::Error>> {
        let regex = Regex::new(&path)?;
        Ok(Self { regex, handler })
    }
}


pub struct Router {
    method_map: HashMap<Method, Vec<RouterHandler>>
}

impl Router {
    pub fn new() -> Self {
        let method_map = HashMap::new();
        Self {method_map}
    }

    pub fn get(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::GET).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }

    pub fn post(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::POST).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }

    pub fn put(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::PUT).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }   
    
    pub fn patch(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::PATCH).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }
    
    pub fn options(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::OPTIONS).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }

    pub fn delete(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::DELETE).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }

    pub fn head(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::HEAD).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }

    pub fn connect(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::CONNECT).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }

    pub fn trace(&mut self, path: &str, handler: Box<dyn Handler>) -> Result<(), Box<dyn std::error::Error>> {
        let route_handler = RouterHandler::builder(path.to_string(), handler)?;
        let m = self.method_map.entry(Method::TRACE).or_insert(Vec::new());
        m.push(route_handler);

        Ok(())
    }

    pub fn get_handler(&self, path: &str, query: Option<&str>, method: &Method) -> Option<RouteMatch> {
        let v = self.method_map.get(method)?;
        let handler = v.iter()
            .find(|handler| {handler.regex.is_match(path)})
            .map(|handler| {handler})?;

        let mut params = Params::new();
        handler.regex.capture_names()
            .for_each(|i| {
                if let Some(name) = i {
                    if let Some(cap) = handler.regex.captures(path) {
                        if let Some(val) = cap.name(name) {
                            params.insert(name.to_string(), val.as_str().to_string())
                        }
                    }
                }
            });

        let query = Query::new(query);
        let route_match = RouteMatch{handler: &*handler.handler, query, params};

        Some(route_match) 
    }
}

pub struct RouteMatch<'a> {
    pub handler: &'a dyn Handler,
    pub params: Params,
    pub query: Query
}

pub async fn route(req: Request<Body>, shared_router: Arc<Router>, shared_service: Arc<Service>) -> Result<Resp, Infallible> {
    let path = req.uri().path();
    let query = req.uri().query();
    let method = req.method();
    match shared_router.get_handler(path, query, method) {
        Some(m) => {
            let ctx = params::Context::new(m.query, m.params, req);
            return Ok(m.handler.invoke(shared_service, ctx).await)
        },
        None => {return Ok("Not Found".into_response())}
    }
}
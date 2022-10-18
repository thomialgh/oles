use std::{sync::Arc, future::Future, collections::HashMap, vec, convert::Infallible};
use hyper::{Method, Request, Body};
use regex::Regex;
use async_trait::async_trait;

use crate::{response::{Resp, IntoResponse, self}, params::{Params, Query, Context}};

pub type FnHandler<S, Fut> = fn(Arc<S>, Context) -> Fut;

pub struct Handler<S, Fut>
where   
    S: Send + Sync + 'static,
    Fut: Future<Output = Resp> + Send + Sync + 'static
{
    regex: Regex,
    f: Box<FnHandler<S, Fut>>
}

impl<S, Fut> Handler<S, Fut> 
where
    S: Send + Sync + 'static,
    Fut: Future<Output = Resp> + Send + Sync + 'static
{

    pub fn new(path: &str, f: FnHandler<S, Fut>) -> Self {
        let regex = regex::Regex::new(path).unwrap();
        Self { regex, f: Box::new(f) }
    }
}

#[async_trait]
pub trait HandlerTrait: Send + Sync + 'static {
    type Svc;

    async fn invoke(&self, s: Arc<Self::Svc>, ctx: Context) -> Resp;
    fn match_path(&self, path: &str) -> bool;
    fn get_params(&self, path: &str) -> Params;
}

#[async_trait]
impl<S, Fut> HandlerTrait for Handler<S, Fut> 
where
    S: Send + Sync + 'static,
    Fut: Future<Output = Resp> + Send + Sync +  'static
{
    type Svc = S;

    async fn invoke(&self, s: Arc<Self::Svc>, ctx: Context) -> Resp {
        (self.f)(s, ctx).await
    }

    fn match_path(&self, path: &str) -> bool {
        self.regex.is_match(path)
    }

    fn get_params(&self, path: &str) -> Params {
        let mut param = Params::new();
        self.regex.capture_names()
        .for_each(|name| {
            if let Some(name) = name {
                if let Some(cap) = self.regex.captures(path) {
                    if let Some(val) = cap.name(name) {
                        param.insert(name.to_string(), val.as_str().to_string())
                    }
                }
            }
        });

        param
    }

} 

pub struct Router<S> 
where
    S: Send + Sync + 'static,
{
    map_route: HashMap<Method, Vec<Box<dyn HandlerTrait<Svc = S>>>>
}

impl<S> Router<S> 
where
    S: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {map_route: HashMap::new()}
    }

    pub fn get(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::GET).or_insert(vec![]);
        m.push(handler);
    }

    pub fn post(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::POST).or_insert(vec![]);
        m.push(handler);
    }

    pub fn put(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::PUT).or_insert(vec![]);
        m.push(handler);
    }

    pub fn delete(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::DELETE).or_insert(vec![]);
        m.push(handler);
    }

    pub fn patch(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::PATCH).or_insert(vec![]);
        m.push(handler);
    }

    pub fn head(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::HEAD).or_insert(vec![]);
        m.push(handler);
    }

    pub fn connect(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::CONNECT).or_insert(vec![]);
        m.push(handler);
    }

    pub fn trace(&mut self, handler: Box<dyn HandlerTrait<Svc = S>>) {
        let m = self.map_route.entry(Method::TRACE).or_insert(vec![]);
        m.push(handler);
    }

    pub fn get_handler(&self, method: &Method, path: &str, query: Option<&str>) -> Option<RouteMatch<S>> {
       let h = self.map_route.get(method)?
        .iter()
        .find(|i|i.match_path(path))?;
       Some(
        RouteMatch{
            param: h.get_params(path),
            query: Query::new(query),
            handler: &**h
           }
       )
    }
}

pub struct RouteMatch<'a, S> {
    pub query: Query,
    pub param: Params,
    pub handler: &'a dyn HandlerTrait<Svc = S>    
}

pub async fn route<S>(req: Request<Body>, shared_router: Arc<Router<S>>, svc: Arc<S>) -> Result<Resp, Infallible>
where
    S: Send + Sync + 'static
{
    let method = req.method();
    let path = req.uri().path();
    let query = req.uri().query();

    match shared_router.get_handler(method, path, query) {
        Some(router_match) => {
            let ctx = Context::new(router_match.query, router_match.param, req);
            return Ok(router_match.handler.invoke(svc, ctx).await)
        }

        None => {
           return Ok(response::response_not_found().await)
        }
    }

}
use std::{convert::Infallible, future::Future};
use hyper::{
    Response, 
    Request, 
    Body, 
    Method,
};
use regex::Regex;

use crate::{
    response::{response_not_found, response_internal_server_err, response_method_not_allowed},
    params::{QueryParams, PathParams, ContextHandler, self}
};

use async_trait::async_trait;


#[async_trait]
pub trait FnHandler: Send + Sync +  'static {
    async fn invoke(&self, ctx: params::ContextHandler, req: Request<Body>) -> Result<Response<Body>, Infallible>; 
}

#[async_trait]
impl <F: Send + Sync + 'static, Fut> FnHandler for F 
where 
    F: Fn(params::ContextHandler, Request<Body>) -> Fut,
    Fut: Future<Output = Result<Response<Body>, Infallible>> + Send + 'static,
{
    async fn invoke(&self, ctx: params::ContextHandler, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        self(ctx, req).await
    }

} 

pub struct Handler
{
    regex_path: Regex,
    method: Method,
    f: Box<dyn FnHandler>
}

impl Handler
{
    pub async fn new(path: &str, method: Method, f: Box<dyn FnHandler>) -> Self {
        let regex_path = Regex::new(format!(r"^{path}$").as_str()).unwrap();
        Self { regex_path, method, f }
    }

    pub async fn get_params(&self, path: &str) -> PathParams {
        let mut path_param = PathParams::new();

        match self.regex_path.captures(path) {
            Some(cap) => {
                self.regex_path.capture_names().for_each(|i| {
                    if let Some(cap_name) = i {
                        if let Some (m) = cap.name(cap_name) {
                            path_param.insert(cap_name, m.as_str());
                        }
                    }
                });
            },
            _ => {}
        };

        path_param
    }
}

pub struct Handlers
{
    pub handlers: Vec<Handler>
}

impl Handlers
{
    pub async fn new(handlers: Vec<Handler>) -> Self {
        Self {handlers}
    }

    pub async fn match_by_path(&self, path: &str) -> Option<Vec<&Handler>> {
        let handlers: Vec<&Handler> = self.handlers
            .iter()
            .filter(|i| i.regex_path.is_match(path))
            .collect();

        match handlers.len() {
            x if x == 0 => None,
            _ => Some(handlers)
        }
    }

    pub async fn find_by_method<'a>(&self, handlers: Vec<&'a Handler>, method: &Method) -> Option<&'a Handler> {
        handlers
            .iter()
            .find(|&i| i.method == *method)
            .map(|&i| i)
    }

    pub async fn route(&self, _req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let path = _req.uri().path();
        let method =  _req.method();
        match self.match_by_path(path).await {
            Some(handlers) => {
                match self.find_by_method(handlers, method).await {
                    Some(handler) => {
                        let query = QueryParams::new(&_req);
                        let params = handler.get_params(path).await;

                        let ctx = ContextHandler::new(params, query);
                        handler.f.invoke(ctx, _req).await
                    },
                    None => Ok(response_method_not_allowed().await.unwrap_or(response_internal_server_err().await))
                }
            },
            None => {Ok(response_not_found().await.unwrap_or(response_internal_server_err().await))}
        }
    } 
}

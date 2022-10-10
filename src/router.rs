use std::{future::Future, pin::Pin, convert::Infallible, vec, error::Error};
use hyper::{Request, Body, Method, Response, service::Service};
use regex::Regex;

use crate::{handler::{HandleFunc, PathParams, QueryParams, ContextHandler}, response::{
    response_not_found,
    response_method_not_allowed,
    response_internal_server_err
}};


#[derive(Debug)]
pub struct Route {
    method: Method,
    path_regex: Regex,
    handler: HandleFunc,

}

impl Route {
    pub  fn route(method: Method, path: &str, handler: HandleFunc) -> Result<Self, Box<dyn Error>> {
        let path_regex = Regex::new(format!(r"^{path}$").as_str())?;
        Ok(
            Self { 
                method,
                handler,
                path_regex
             }
        )
    }

    pub fn get_handler(&self) -> HandleFunc {
        self.handler
    }

    pub fn match_method(&self, method: &Method) -> bool {
        self.method.as_str() == method.as_str()
    }

    pub fn match_path(&self, path: &str) -> bool {
        self.path_regex.is_match(path)
    }

    pub fn get_params(&self, path: &str) -> PathParams {
        let mut params = PathParams::new();

        if let Some(cap) = self.path_regex.captures(path) {
            self.path_regex.capture_names().for_each(|n| {
                match n {
                    Some(name) => {
                        if let Some(val) = cap.name(name) {
                            params.insert(name, val.as_str())
                        }
                    },
                    _ => {},
                }
            })
        }

        params
    }

}
#[derive(Debug)]
pub struct Router {
     pub route : Vec<Route>
}

impl Router {
    pub fn new() -> Self {
        Self{route:vec![]}
    }

    pub fn get(&mut self, path: &str, handler: HandleFunc) -> Result<(), Box<dyn Error>> {
        let route = Route::route(Method::GET, path, handler)?;
        self.route.push(route);
        Ok(())
    }

    pub fn post(&mut self, path: &str, handler: HandleFunc) -> Result<(), Box<dyn Error>> {
        let route = Route::route(Method::POST, path, handler)?;
        self.route.push(route);
        Ok(())
    }

    pub fn put(&mut self, path: &str, handler: HandleFunc) -> Result<(), Box<dyn Error>> {
        let route = Route::route(Method::PUT, path, handler)?;
        self.route.push(route);
        Ok(())
    }

    pub fn patch(&mut self, path: &str, handler: HandleFunc) -> Result<(), Box<dyn Error>> {
        let route = Route::route(Method::PUT, path, handler)?;
        self.route.push(route);
        Ok(())
    }

    pub fn delete(&mut self, path: &str, handler: HandleFunc) -> Result<(), Box<dyn Error>> {
        let route = Route::route(Method::DELETE, path, handler)?;
        self.route.push(route);
        Ok(())
    }

    pub fn option(&mut self, path: &str, handler: HandleFunc) -> Result<(), Box<dyn Error>> {
        let route = Route::route(Method::OPTIONS, path, handler)?;
        self.route.push(route);
        Ok(())
    }

    fn get_by_path(&self, path: &str) -> Vec<&Route> {
        self.route.iter().filter(|&i| i.match_path(path)).collect()
    }

    fn get_route_by_method<'a>(&self, v: &Vec<&'a Route>, method: &Method) -> Option<&'a Route> {
        v.iter().find(|&i| i.match_method(method)).map(|&i| i)
    }


}

#[derive(Debug)]
pub struct RouterService {
    _router: Router
}

impl RouterService {
    pub fn new(router: Router) -> Self {
        Self{_router: router}
    }
}

impl Service<Request<Body>> for RouterService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let path = _req.uri().path();
        let method = _req.method();
        let route = self._router.get_by_path(path);

        match route.len() {
            x if x == 0 => {
                let fut = async {
                    let resp = response_not_found().unwrap_or(response_internal_server_err());
                    Ok::<Self::Response, Self::Error>(resp)
                };
    
                return Box::pin(fut);
            },
            _ => {

                match self._router.get_route_by_method(&route, method) {
                    Some(r) => {

                        let params = r.get_params(path);
                        let query = QueryParams::new(&_req);

                        let ctx = ContextHandler::new(params, query);
                        let h = r.handler.clone();
                        let fut = async move {
                            h(ctx, _req)
                        };

                    return Box::pin(fut);
                },
                _ => {
                    let fut = async {
                        let resp = response_method_not_allowed().unwrap_or(response_internal_server_err());
                            Ok::<Self::Response, Self::Error>(resp)
                        };
    
                        return Box::pin(fut);
                    }
                }

            }

        }
        
    }

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(())) 
    }
}

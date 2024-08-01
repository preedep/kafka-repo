use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, Result};
use actix_web::dev::{Transform, Service};
use futures_util::future::{ok, Ready};
use futures_util::future::{LocalBoxFuture, FutureExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::rc::Rc;
use std::cell::RefCell;
use std::task::{Context, Poll};
use log::debug;
use crate::entities::{AuthError, Claims};

pub struct JwtMiddleware {
    secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: String) -> Self {
        JwtMiddleware { secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareMiddleware {
            service: Rc::new(RefCell::new(service)),
            secret: self.secret.clone(),
        })
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: Rc<RefCell<S>>,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.borrow_mut().poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone();
        let svc = self.service.clone();

        async move {
            return if req.path().starts_with("/api/v1") {
                debug!("Checking JWT token");
                if let Some(authen_header) = req.headers().get("Authorization") {
                    if let Ok(authen_str) = authen_header.to_str() {
                        if authen_str.starts_with("Bearer ") {
                            let token = &authen_str[7..];
                            let validation = Validation::default();
                            let token_data = decode::<Claims>(
                                token,
                                &DecodingKey::from_secret(secret.as_ref()),
                                &validation,
                            );

                            return match token_data {
                                Ok(data) => {
                                    req.extensions_mut().insert(data.claims);
                                    svc.call(req).await
                                }
                                Err(_) => Err(AuthError::Unauthorized.into()),
                            }
                        }
                    }
                }

                Err(AuthError::Unauthorized.into())
            } else {
                svc.call(req).await
            }
        }
            .boxed_local()
    }
}